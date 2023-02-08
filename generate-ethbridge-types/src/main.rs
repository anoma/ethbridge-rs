use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;
use ethers_contract::Abigen;
use eyre::{eyre as err, WrapErr};
use itertools::Itertools;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

struct Paths {
    /// Path to the output directory of the generated crates.
    output_dir: PathBuf,
    /// Path to the ABI files directory.
    abi_files_dir: PathBuf,
}

/// Generate Ethereum bridge Rust types compatible with Namada's
/// Rust code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory to parse ABI files from
    #[arg(short = 'p', long, default_value_t = String::from("target"))]
    abi_files_dir: String,

    /// The git tag of `ethereum-bridge` whose artifacts
    /// we are to download. If no tag is provided, we try
    /// to use files present in the specified ABI files
    /// directory
    #[arg(short = 't', long)]
    ethereum_bridge_tag: Option<String>,

    /// Path to the output directory of the generated crates.
    /// If no output directory is specified, the current working
    /// directory is used instead
    #[arg(short = 'o', long)]
    output_dir: Option<String>,

    /// The version of the generated crates. If not specified, the
    /// version of this CLI command is used instead
    #[arg(short = 'v', long)]
    crate_version: Option<String>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}

fn run() -> eyre::Result<()> {
    let Args {
        output_dir,
        abi_files_dir,
        ethereum_bridge_tag,
        crate_version,
    } = Args::parse();

    let crate_version = crate_version.unwrap_or_else(|| env!("CARGO_PKG_VERSION").into());
    let paths = Paths {
        output_dir: output_dir.map(|s| s.into()).unwrap_or_else(PathBuf::new),
        abi_files_dir: abi_files_dir.into(),
    };
    if let Some(tag) = ethereum_bridge_tag {
        download_abi_files(tag, &paths)?;
    }

    let mut structs = BTreeMap::new();

    generate_crates("Bridge", &crate_version, &paths, &mut structs)?;
    generate_crates("Governance", &crate_version, &paths, &mut structs)?;

    generate_crate_template(
        "ethbridge-structs".into(),
        &crate_version,
        vec![("ethers".into(), "1.0.2".into())],
        &paths,
    )?;
    generate_crate_source(
        "ethbridge-structs".into(),
        &paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
        })
        .chain(structs.into_values().flatten()),
    )?;

    Ok(())
}

fn generate_crates(
    abi_file: &str,
    version: &str,
    paths: &Paths,
    structs: &mut BTreeMap<String, Vec<TokenStream>>,
) -> eyre::Result<()> {
    let abi_file_path = paths.abi_files_dir.join(format!("{abi_file}.abi"));
    let abi_gen = Abigen::from_file(&abi_file_path)
        .with_context(|| format!("file not found: {}", abi_file_path.display()))?;
    let (abi, _) = abi_gen.expand()?;
    let mut structs_iter = abi.abi_structs.into_iter();
    loop {
        let Some(tt) = structs_iter.next() else {
            break;
        };
        let mut tts = vec![tt.into()];
        for _ in 0..5 {
            tts.push(
                structs_iter
                    .next()
                    .ok_or_else(|| err!("insufficient token trees in generated rust code"))?
                    .into(),
            );
        }
        let Some(TokenTree::Ident(ident)) = structs_iter.next() else {
            eyre::bail!("expected identifier in generated rust code, but got something else");
        };
        let key = ident.to_string();
        tts.push(TokenTree::Ident(ident).into());
        tts.push(
            structs_iter
                .next()
                .ok_or_else(|| err!("struct definition not found in generated rust code"))?
                .into(),
        );
        structs.insert(key, tts);
    }
    generate_crate_template(
        get_subcrate(abi_file, "calls"),
        version,
        vec![
            ("ethbridge-structs".into(), String::new()),
            ("ethers".into(), "1.0.2".into()),
        ],
        paths,
    )?;
    generate_crate_source(
        get_subcrate(abi_file, "calls"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            use ::ethbridge_structs::*;
        })
        .chain(abi.call_structs.into_iter().map(|tt| tt.into())),
    )?;
    generate_crate_template(
        get_subcrate(abi_file, "events"),
        version,
        vec![
            ("ethbridge-structs".into(), String::new()),
            ("ethers".into(), "1.0.2".into()),
        ],
        paths,
    )?;
    generate_crate_source(
        get_subcrate(abi_file, "events"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            use ::ethbridge_structs::*;
        })
        .chain(abi.events.into_iter().map(|tt| tt.into())),
    )?;
    generate_crate_template(
        get_subcrate(abi_file, "contract"),
        version,
        vec![
            dispatch_on_abi(
                abi_file,
                || ("ethbridge-bridge-calls".into(), String::new()),
                || ("ethbridge-governance-calls".into(), String::new()),
            ),
            dispatch_on_abi(
                abi_file,
                || ("ethbridge-bridge-events".into(), String::new()),
                || ("ethbridge-governance-events".into(), String::new()),
            ),
            ("ethbridge-structs".into(), String::new()),
            ("ethers".into(), "1.0.2".into()),
        ],
        paths,
    )?;
    generate_crate_source(
        get_subcrate(abi_file, "contract"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            #![allow(clippy::too_many_arguments)]
            use ::ethbridge_structs::*;
        })
        .chain(
            dispatch_on_abi(
                abi_file,
                || {
                    quote! {
                        use ::ethbridge_bridge_calls::*;
                        use ::ethbridge_bridge_events::*;
                    }
                },
                || {
                    quote! {
                        use ::ethbridge_governance_calls::*;
                        use ::ethbridge_governance_events::*;
                    }
                },
            )
            .into_iter()
            .map(|tt| tt.into()),
        )
        .chain(abi.contract.into_iter().map(|tt| tt.into())),
    )?;
    Ok(())
}

fn dispatch_on_abi<T, F1, F2>(abi_file: &str, mut bridge: F1, mut governance: F2) -> T
where
    F1: FnMut() -> T,
    F2: FnMut() -> T,
{
    match abi_file {
        "Bridge" => bridge(),
        "Governance" => governance(),
        _ => unreachable!("unknown ABI file type"),
    }
}

fn generate_crate_template(
    crate_name: String,
    crate_version: &str,
    deps: Vec<(String, String)>,
    paths: &Paths,
) -> eyre::Result<()> {
    let deps = deps
        .into_iter()
        .map(|(dep, ver)| {
            if !dep.starts_with("ethbridge-") {
                format!("{dep} = \"{ver}\"")
            } else {
                format!("{dep} = {{ path = \"../{dep}\" }}")
            }
        })
        .join("\n");
    let crate_path = paths.output_dir.join(&crate_name);
    let cargo_toml_path = crate_path.join("Cargo.toml");
    std::fs::create_dir_all(crate_path.join("src"))
        .with_context(|| format!("failed to create directory: {}", crate_path.display()))?;
    let err = std::fs::write(
        &cargo_toml_path,
        format!(
            "\
[package]
name = \"{crate_name}\"
version = \"{crate_version}\"
edition = \"2021\"

[dependencies]
{deps}
"
        ),
    );
    err.with_context(|| format!("failed to create file: {}", cargo_toml_path.display()))
}

fn generate_crate_source(
    crate_name: String,
    paths: &Paths,
    source: impl IntoIterator<Item = TokenStream>,
) -> eyre::Result<()> {
    let lib_path = paths.output_dir.join(crate_name).join("src").join("lib.rs");
    let source = source
        .into_iter()
        .fold(TokenStream::new(), |mut stream, other| {
            stream.extend(other);
            stream
        })
        .to_string();
    std::fs::write(&lib_path, source)
        .with_context(|| format!("failed to create file: {}", lib_path.display()))
}

fn get_subcrate(abi_file: &str, suffix: &str) -> String {
    format!("ethbridge-{}-{suffix}", abi_file.to_lowercase())
}

fn download_abi_files(_tag: String, _paths: &Paths) -> eyre::Result<()> {
    eyre::bail!("downloading of ABI artifacts is not implemented yet")
}
