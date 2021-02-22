use anyhow::Context as _;
use structopt::StructOpt as _;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

mod models;
mod util;

use crate::models::{cargo_toml, directory_list::DirectoryList, rest_description::RestDescription};

#[derive(Debug, structopt::StructOpt)]
enum Cli {
    /// Remove the `DirectoryList`, all `RestDescription`s, and all generated crates
    Clean {
        /// Path to the `DirectoryList`
        #[structopt(long, name = "DIRECTORY_LIST_PATH", default_value = "./discovery.json")]
        directory_list_path: std::path::PathBuf,
        /// Output parent directory
        #[structopt(long, name = "OUTPUT_DIR", default_value = "./crates")]
        output_dir: std::path::PathBuf,
        /// Really do it, without CLI prompt
        #[structopt(short, long)]
        yes: bool,
    },
    /// Download the `DirectoryList` via the Google API Discovery Service
    DownloadDirectoryList {
        /// Path to the `DirectoryList`
        #[structopt(long, name = "DIRECTORY_LIST_PATH", default_value = "./discovery.json")]
        directory_list_path: std::path::PathBuf,
        /// Endpoint for the Google API Discovery Service "list" operation
        #[structopt(
            long,
            name = "URL",
            default_value = "https://discovery.googleapis.com/discovery/v1/apis"
        )]
        endpoint: String,
    },
    /// Download `RestDescription`s via the Google API Discovery Service
    ///
    /// Endpoints are discovered from the `DirectoryList`
    DownloadRestDescription {
        /// Path to the `DirectoryList`
        #[structopt(long, name = "DIRECTORY_LIST_PATH", default_value = "./discovery.json")]
        directory_list_path: std::path::PathBuf,
        /// APIs to download descriptions for (e.g. pagespeedonline:v5)
        ///
        /// All APIs in the `DirectoryList` will be discovered if not provided.
        /// Overrides --exclude
        #[structopt(name = "ID", overrides_with = "EXCLUDE_ID")]
        ids: Vec<String>,
        /// Output parent directory
        ///
        /// The directory structure will be generated as "${OUTPUT_DIR}/${API}/${VERSION}/
        /// ${REST_DESCRIPTION_PATH}"
        #[structopt(long, name = "OUTPUT_DIR", default_value = "./crates")]
        output_dir: std::path::PathBuf,
        /// Path to the `RestDescription`
        ///
        /// The `RestDescription` will be saved to this subpath in each subdirectory.
        /// See `--output-dir` for more information
        #[structopt(
            short,
            long,
            name = "REST_DESCRIPTION_PATH",
            default_value = "discovery.json"
        )]
        rest_description_path: std::path::PathBuf,
    },
    /// Generate Rust crates for `RestDescription`s
    Generate {
        /// Path to the `DirectoryList`
        #[structopt(long, name = "DIRECTORY_LIST_PATH", default_value = "./discovery.json")]
        directory_list_path: std::path::PathBuf,
        /// APIs to generate Rust crates for (e.g. pagespeedonline:v5)
        ///
        /// All crates in the `DirectoryList` will be generated if not provided.
        /// Overrides --exclude
        #[structopt(name = "ID", overrides_with = "EXCLUDE_ID")]
        ids: Vec<String>,
        /// Output parent directory
        ///
        /// The directory structure will be generated as "${OUTPUT_DIR}/${API}/${VERSION}".
        /// There is expected to be a `RestDescription` at "${OUTPUT_DIR}/${API}/${VERSION}
        /// ${REST_DESCRIPTION_PATH}"
        #[structopt(long, name = "OUTPUT_DIR", default_value = "./crates")]
        output_dir: std::path::PathBuf,
        /// Path to the `RestDescription`
        ///
        /// The `RestDescription` is expected to be found at this subpath in each subdirectory.
        /// See `--output-dir` for more information
        #[structopt(short, long, name = "INPUT_PATH", default_value = "discovery.json")]
        rest_description_path: std::path::PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;
    let opt = Cli::from_args();
    let client = reqwest::Client::new();

    match opt {
        Cli::Clean {
            directory_list_path,
            output_dir,
            yes,
        } => {
            {
                let confirmed = yes
                    || dialoguer::Confirm::new()
                        .wait_for_newline(true)
                        .with_prompt(format!("Really remove {:?}?", &*directory_list_path))
                        .interact()?;
                if confirmed {
                    log::warn!("removing {:?}", &*directory_list_path);
                    tokio::fs::remove_file(&*directory_list_path).await.ok();
                } else {
                    log::warn!("aborting");
                    std::process::exit(1);
                }
            }

            {
                let confirmed = yes
                    || dialoguer::Confirm::new()
                        .wait_for_newline(true)
                        .with_prompt(format!("Really remove {:?}?", &*output_dir))
                        .interact()?;
                if confirmed {
                    log::warn!("removing {:?}", &*output_dir);
                    tokio::fs::remove_dir_all(&*output_dir).await.ok();
                } else {
                    log::warn!("aborting");
                    std::process::exit(1);
                }
            }

            Ok(())
        }
        Cli::DownloadDirectoryList {
            directory_list_path,
            endpoint,
        } => {
            log::debug!("reqwesting `DirectoryList`");
            let directory_list = client.get(&*endpoint).send().await?;

            if !directory_list.status().is_success() {
                log::error!(
                    "GET {} HTTP/1.1 -> {:?} {}",
                    &*endpoint,
                    directory_list.version(),
                    directory_list.status()
                );
                std::process::exit(1);
            }

            log::debug!("decoding `DirectoryList`");
            let directory_list =
                directory_list
                    .json::<DirectoryList>()
                    .await
                    .with_context(|| {
                        log::error!("error decoding `DirectoryList`");
                        "error decoding `DirectoryList`"
                    })?;

            log::debug!("writing `DirectoryList` to {:?}", &*directory_list_path);
            tokio::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&*directory_list_path)
                .await
                .with_context(|| {
                    log::error!("error opening {:?}", &*directory_list_path);
                    format!("error opening {:?}", &*directory_list_path)
                })?
                .write_all(&*serde_json::to_vec_pretty(&directory_list)?)
                .await
                .with_context(|| {
                    log::error!("error writing {:?}", &*directory_list_path);
                    format!("error writing {:?}", &*directory_list_path)
                })?;

            log::info!("wrote `DirectoryList` to {:?}", &*directory_list_path);
            Ok(())
        }
        Cli::DownloadRestDescription {
            directory_list_path,
            ids,
            output_dir,
            rest_description_path,
        } => {
            let directory_list: DirectoryList = {
                let mut buf = Vec::new();

                log::debug!("reading `DirectoryList` from {:?}", &*directory_list_path);
                tokio::fs::OpenOptions::new()
                    .read(true)
                    .open(&*directory_list_path)
                    .await
                    .with_context(|| {
                        log::error!("error opening {:?}", &*directory_list_path);
                        format!("error opening {:?}", &*directory_list_path)
                    })?
                    .read_to_end(&mut buf)
                    .await
                    .with_context(|| {
                        log::error!("error reading {:?}", &*directory_list_path);
                        format!("error reading {:?}", &*directory_list_path)
                    })?;

                log::debug!("decoding `DirectoryList`");
                serde_json::from_slice(&buf)?
            };

            let all_directory_items = directory_list.items.ok_or_else(|| {
                log::error!("`DirectoryList.items` was `None`");
                anyhow::anyhow!("`DirectoryList.items` was `None`")
            })?;

            let directory_items = if ids.len() > 0 {
                let mut buf = Vec::new();

                for id in ids {
                    if let Some(item) = all_directory_items.iter().find(|item| item.id == id) {
                        buf.push(item);
                    } else {
                        log::warn!("cannot include {}, not found in `DirectoryList.items`", id);
                    }
                }

                buf
            } else {
                all_directory_items.iter().collect()
            };

            if directory_items.len() == 0 {
                log::warn!("no `DirectoryItem`s in filtered list");
            }

            let results = futures::future::join_all(directory_items.into_iter().map(|item| {
                let client = client.clone();

                let rest_description_path = output_dir
                    .join(change_case::snake_case(&*item.name))
                    .join(change_case::snake_case(&*item.version))
                    .join(&*rest_description_path);

                let crate_name = format!(
                    "{}_{}",
                    change_case::snake_case(&*item.name),
                    change_case::snake_case(&*item.version)
                );

                async move {
                    let rest_description_url =
                        item.discovery_rest_url.as_ref().ok_or_else(|| {
                            log::error!("`DirectoryItem.discovery_rest_url` was `None`");
                            anyhow::anyhow!("`DirectoryItem.discovery_rest_url` was `None`")
                        })?;

                    log::debug!("{}: reqwesting `RestDescription`", &*crate_name);
                    let rest_description = client.get(&*rest_description_url).send().await?;

                    if !rest_description.status().is_success() {
                        log::error!(
                            "GET {} HTTP/1.1 -> {:?} {}",
                            &*rest_description_url,
                            rest_description.version(),
                            rest_description.status()
                        );
                        anyhow::bail!(
                            "GET {} HTTP/1.1 -> {:?} {}",
                            &*rest_description_url,
                            rest_description.version(),
                            rest_description.status()
                        );
                    }

                    log::debug!("{}: decoding `RestDescription`", &*crate_name);
                    let rest_description = rest_description
                        .json::<RestDescription>()
                        .await
                        .with_context(|| {
                            log::error!("{}: error decoding `RestDescription`", &*crate_name);
                            format!("{}: error decoding `RestDescription`", &*crate_name)
                        })?;

                    {
                        let output_dir = rest_description_path.parent().unwrap();

                        log::debug!("{}: creating directory {:?}", &*crate_name, output_dir);

                        tokio::fs::create_dir_all(output_dir)
                            .await
                            .with_context(|| {
                                log::error!(
                                    "{}: could not create directory {:?}",
                                    &*crate_name,
                                    output_dir
                                );
                                format!(
                                    "{}: could not create directory {:?}",
                                    &*crate_name, output_dir
                                )
                            })?;
                    }

                    log::debug!(
                        "{}: writing `RestDescription` to {:?}",
                        &*crate_name,
                        &*rest_description_path
                    );
                    tokio::fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(&*rest_description_path)
                        .await
                        .with_context(|| {
                            log::error!(
                                "{}: error opening {:?}",
                                &*crate_name,
                                &*rest_description_path
                            );
                            format!(
                                "{}: error opening {:?}",
                                &*crate_name, &*rest_description_path
                            )
                        })?
                        .write_all(&*serde_json::to_vec_pretty(&rest_description)?)
                        .await
                        .with_context(|| {
                            log::error!(
                                "{}: error writing {:?}",
                                &*crate_name,
                                &*rest_description_path
                            );
                            format!(
                                "{}: error writing {:?}",
                                &*crate_name, &*rest_description_path
                            )
                        })?;

                    log::info!(
                        "{}: wrote `RestDescription` to {:?}",
                        &*crate_name,
                        &*rest_description_path
                    );
                    Ok::<_, anyhow::Error>(())
                }
            }))
            .await;

            results.into_iter().collect::<Result<_, _>>()
        }
        Cli::Generate {
            directory_list_path,
            ids,
            output_dir,
            rest_description_path,
        } => {
            let directory_list: DirectoryList = {
                let mut buf = Vec::new();

                log::debug!("reading `DirectoryList` from {:?}", &*directory_list_path);
                tokio::fs::OpenOptions::new()
                    .read(true)
                    .open(&*directory_list_path)
                    .await
                    .with_context(|| {
                        log::error!("error opening {:?}", &*directory_list_path);
                        format!("error opening {:?}", &*directory_list_path)
                    })?
                    .read_to_end(&mut buf)
                    .await
                    .with_context(|| {
                        log::error!("error reading {:?}", &*directory_list_path);
                        format!("error reading {:?}", &*directory_list_path)
                    })?;

                log::debug!("decoding `DirectoryList`");
                serde_json::from_slice(&buf)?
            };

            let all_directory_items = directory_list.items.ok_or_else(|| {
                log::error!("`DirectoryList.items` was `None`");
                anyhow::anyhow!("`DirectoryList.items` was `None`")
            })?;

            let directory_items = if ids.len() > 0 {
                let mut buf = Vec::new();

                for id in ids {
                    if let Some(item) = all_directory_items.iter().find(|item| item.id == id) {
                        buf.push(item);
                    } else {
                        log::warn!("cannot include {}, not found in `DirectoryList.items`", id);
                    }
                }

                buf
            } else {
                all_directory_items.iter().collect()
            };

            if directory_items.len() == 0 {
                log::warn!("no `DirectoryItem`s in filtered list");
            }

            let results = futures::future::join_all(directory_items.into_iter().map(|item| {
                let rest_description_path = output_dir
                    .join(change_case::snake_case(&*item.name))
                    .join(change_case::snake_case(&*item.version))
                    .join(&*rest_description_path);

                let crate_name = format!(
                    "{}_{}",
                    change_case::snake_case(&*item.name),
                    change_case::snake_case(&*item.version)
                );

                let crate_path = rest_description_path.parent().unwrap();
                let cargo_toml_path = crate_path.join("Cargo.toml");
                let lib_rs_path = crate_path.join("src").join("lib.rs");

                async move {
                    let rest_description: RestDescription = {
                        let mut buf = Vec::new();

                        log::debug!(
                            "{}: reading `RestDescription` from {:?}",
                            &*crate_name,
                            &*rest_description_path
                        );
                        tokio::fs::OpenOptions::new()
                            .read(true)
                            .open(&*rest_description_path)
                            .await
                            .with_context(|| {
                                log::error!(
                                    "{}: error opening {:?}",
                                    &*crate_name,
                                    &*rest_description_path
                                );
                                format!(
                                    "{}: error opening {:?}",
                                    &*crate_name, &*rest_description_path
                                )
                            })?
                            .read_to_end(&mut buf)
                            .await
                            .with_context(|| {
                                log::error!(
                                    "{}: error reading {:?}",
                                    &*crate_name,
                                    &*rest_description_path
                                );
                                format!(
                                    "{}: error reading {:?}",
                                    &*crate_name, &*rest_description_path
                                )
                            })?;

                        log::debug!("{}: decoding `RestDescription`", &*crate_name);
                        serde_json::from_slice(&buf)?
                    };

                    let schemas = rest_description.schemas.ok_or_else(|| {
                        log::error!("{}: `RestDescription.schemas` was `None`", &*crate_name);
                        anyhow::anyhow!("{}: `RestDescription.schemas` was `None`", &*crate_name)
                    })?;

                    log::debug!("{}: constructing ast", &*crate_name);
                    let tokens = schemas
                        .iter()
                        .map(|(schema_name, schema)| {
                            log::debug!("{}::{}: constructing ast", &*crate_name, &*schema_name);

                            let schema_doc = if let Some(ref description) = schema.description {
                                quote::quote!(#[doc = #description])
                            } else {
                                quote::quote!()
                            };

                            let schema_ident = quote::format_ident!("{}", schema_name);

                            let schema_properties =
                                schema.properties.as_ref().ok_or_else(|| {
                                    log::error!(
                                        "{}::{}: `schema.properties` was `None`",
                                        &*crate_name,
                                        &*schema_name
                                    );
                                    anyhow::anyhow!(
                                        "{}::{}: `schema.properties` was `None`",
                                        &*crate_name,
                                        &*schema_name
                                    )
                                })?;

                            let schema_fields = schema_properties
                                .iter()
                                .map(|(property_name, property)| {
                                    let property_doc =
                                        if let Some(ref description) = property.description {
                                            quote::quote!(#[doc = #description])
                                        } else {
                                            quote::quote!()
                                        };

                                    lazy_static::lazy_static! {
                                        static ref RE_RESERVED: regex::Regex =
                                            regex::Regex::new(r"(?x)^(
                                                    as|
                                                    async|
                                                    await|
                                                    break|
                                                    const|
                                                    continue|
                                                    crate|
                                                    dyn|
                                                    else|
                                                    enum|
                                                    extern|
                                                    false|
                                                    final|
                                                    fn|
                                                    for|
                                                    if|
                                                    impl|
                                                    in|
                                                    let|
                                                    loop|
                                                    macro|
                                                    match|
                                                    mod|
                                                    move|
                                                    mut|
                                                    pub|
                                                    ref|
                                                    return|
                                                    self|
                                                    static|
                                                    struct|
                                                    super|
                                                    trait|
                                                    true|
                                                    type|
                                                    union|
                                                    unsafe|
                                                    use|
                                                    where|
                                                    while
                                                )$").unwrap();
                                    }

                                    let property_ident = change_case::snake_case(&*property_name);
                                    let property_ident =
                                        RE_RESERVED.replace_all(&*property_ident, "_${1}");

                                    let serde_rename = if property_name
                                        != &change_case::camel_case(&*property_ident)
                                        || &*property_ident
                                            != &change_case::snake_case(&*property_name)
                                    {
                                        quote::quote!(#[serde(rename = #property_name)])
                                    } else {
                                        quote::quote!()
                                    };

                                    let property_ident = quote::format_ident!("{}", property_ident);

                                    let property_ty = util::property_ty(property)?;
                                    let property_ty = if property.required == Some(true) {
                                        property_ty
                                    } else {
                                        quote::quote!(Option<#property_ty>)
                                    };

                                    Ok::<_, anyhow::Error>(quote::quote! {
                                        #property_doc
                                        #serde_rename
                                        pub #property_ident: #property_ty
                                    })
                                })
                                .collect::<Result<Vec<_>, _>>()
                                .with_context(|| {
                                    log::error!(
                                        "{}::{}: error constructing ast",
                                        &*crate_name,
                                        &*schema_name
                                    );
                                    format!(
                                        "{}::{}: error constructing ast",
                                        &*crate_name, &*schema_name
                                    )
                                })?;

                            Ok::<_, anyhow::Error>(quote::quote! {
                                #[serde_with::skip_serializing_none]
                                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                                #schema_doc
                                #[serde(deny_unknown_fields)]
                                #[serde(rename_all = "camelCase")]
                                pub struct #schema_ident {
                                    #(#schema_fields),*
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .with_context(|| {
                            log::error!("{}: error constructing ast", &*crate_name,);
                            format!("{}: error constructing ast", &*crate_name,)
                        })?;

                    let tokens = quote::quote!(#(#tokens)*);
                    if tokens.is_empty() {
                        log::warn!("{}: no code generated", &*crate_name);
                    }

                    {
                        let output_dir = lib_rs_path.parent().unwrap();

                        log::debug!("{}: creating directory {:?}", &*crate_name, output_dir);

                        tokio::fs::create_dir_all(output_dir)
                            .await
                            .with_context(|| {
                                log::error!(
                                    "{}: could not create directory {:?}",
                                    &*crate_name,
                                    output_dir
                                );
                                format!(
                                    "{}: could not create directory {:?}",
                                    &*crate_name, output_dir
                                )
                            })?;
                    }

                    log::debug!("{}: writing `lib.rs` to {:?}", &*crate_name, &*lib_rs_path);
                    tokio::fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(&*lib_rs_path)
                        .await
                        .with_context(|| {
                            log::error!("{}: error opening {:?}", &*crate_name, &*lib_rs_path);
                            format!("{}: error opening {:?}", &*crate_name, &*lib_rs_path)
                        })?
                        .write_all(tokens.to_string().as_bytes())
                        .await
                        .with_context(|| {
                            log::error!("{}: error writing {:?}", &*crate_name, &*lib_rs_path);
                            format!("{}: error writing {:?}", &*crate_name, &*lib_rs_path)
                        })?;

                    log::info!(
                        "{}: wrote `lib.rs` to {:?}",
                        &*crate_name,
                        &*rest_description_path
                    );

                    {
                        log::debug!(
                            "{}: formatting {:?} with rustfmt",
                            &*crate_name,
                            &*lib_rs_path
                        );
                        let rustfmt_output = tokio::process::Command::new(
                            std::env::var_os("RUSTFMT").unwrap_or("rustfmt".into()),
                        )
                        .args(&["--edition", "2018"])
                        .arg(lib_rs_path)
                        .env_remove("RUST_LOG")
                        .output()
                        .await
                        .with_context(|| {
                            log::error!("{}: error running rustfmt", &*crate_name);
                            format!("{}: error running rustfmt", &*crate_name)
                        })?;

                        if rustfmt_output.stderr.len() == 0 {
                            log::info!("{}: formatted `lib.rs` with rustfmt", &*crate_name,);
                        } else {
                            log::error!("{}: rustfmt reported invalid syntax", &*crate_name);
                            anyhow::bail!("{}: rustfmt reported invalid syntax", &*crate_name);
                        }
                    }

                    {
                        let crate_name_long = format!("google_api_rust_client_{}", &*crate_name);

                        let crate_description =
                            rest_description.description.unwrap_or(String::new());

                        let crate_version = format!(
                            "{}+{}",
                            env!("CARGO_PKG_VERSION"),
                            &*rest_description.revision
                        );

                        log::debug!(
                            "{}: writing `Cargo.toml` to {:?}",
                            &*crate_name,
                            &*cargo_toml_path
                        );
                        tokio::fs::OpenOptions::new()
                            .create(true)
                            .truncate(true)
                            .write(true)
                            .open(&*cargo_toml_path)
                            .await
                            .with_context(|| {
                                log::error!(
                                    "{}: error opening {:?}",
                                    &*crate_name,
                                    &*cargo_toml_path
                                );
                                format!("{}: error opening {:?}", &*crate_name, &*cargo_toml_path)
                            })?
                            .write_all(&*toml::to_vec(&toml::toml! {
                                [package]
                                name = crate_name_long
                                description = crate_description
                                version = crate_version
                                authors = ["Ezra Celli <ezra@ezracelli.com>"]
                                edition = "2018"

                                [dependencies]
                                serde = { version = "1", features = ["derive"] }
                                serde_json = "1"
                                serde_with = "1"
                            })?)
                            .await
                            .with_context(|| {
                                log::error!(
                                    "{}: error writing {:?}",
                                    &*crate_name,
                                    &*cargo_toml_path
                                );
                                format!("{}: error writing {:?}", &*crate_name, &*cargo_toml_path)
                            })?;

                        log::info!(
                            "{}: wrote `Cargo.toml` to {:?}",
                            &*crate_name,
                            &*rest_description_path
                        );
                    }

                    Ok::<_, anyhow::Error>(())
                }
            }))
            .await;

            results.into_iter().collect::<Result<_, _>>()?;

            let cargo_toml_path = std::path::PathBuf::from(".").join("Cargo.toml");
            let mut cargo_toml: cargo_toml::Manifest = {
                let mut buf = Vec::new();

                log::debug!("reading `Cargo.toml` from {:?}", &*cargo_toml_path);
                tokio::fs::OpenOptions::new()
                    .read(true)
                    .open(&*cargo_toml_path)
                    .await
                    .with_context(|| {
                        log::error!("error opening {:?}", &*cargo_toml_path);
                        format!("error opening {:?}", &*cargo_toml_path)
                    })?
                    .read_to_end(&mut buf)
                    .await
                    .with_context(|| {
                        log::error!("error reading {:?}", &*cargo_toml_path);
                        format!("error reading {:?}", &*cargo_toml_path)
                    })?;

                log::debug!("decoding `Cargo.toml`");
                toml::from_slice(&buf)?
            };

            println!("{:?}", cargo_toml);

            {
                let workspace = cargo_toml.workspace.get_or_insert(cargo_toml::Workspace::default());
                let members = workspace.members.get_or_insert(Vec::default());

                *members = vec![
                    String::from("gen"),
                    std::fs::canonicalize(output_dir)?
                        .strip_prefix(std::env::current_dir()?)?
                        .join("**/*")
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_owned()
                ];

                members.sort();
            }

            log::debug!("writing `Cargo.toml` to {:?}", &*cargo_toml_path);
            tokio::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&*cargo_toml_path)
                .await
                .with_context(|| {
                    log::error!("error opening {:?}", &*cargo_toml_path);
                    format!("error opening {:?}", &*cargo_toml_path)
                })?
                .write_all(&*toml::to_vec(&cargo_toml)?)
                .await
                .with_context(|| {
                    log::error!("error writing {:?}", &*cargo_toml_path);
                    format!("error writing {:?}", &*cargo_toml_path)
                })?;

            log::info!("wrote `Cargo.toml` to {:?}", &*cargo_toml_path);

            Ok(())
        }
    }

    // if let Some(items) = directory_list.items {
    //     let results = futures::future::join_all(items.into_iter().map(|item| {
    //         let client = client.clone();

    //         async move {
    //             if let Some(discovery_rest_url) = item.discovery_rest_url {
    //                 let crate_path = std::env::current_dir()?
    //                     .join("crates")
    //                     .join(change_case::snake_case(&*item.name))
    //                     .join(change_case::snake_case(&*item.version));

    //                 let crate_name = format!(
    //                     "{}-{}",
    //                     change_case::snake_case(&*item.name),
    //                     change_case::snake_case(&*item.version)
    //                 );

    //                 tokio::fs::create_dir_all(&*crate_path).await?;

    //                 let rest_description = client.get(&*discovery_rest_url).send().await?;

    //                 if !rest_description.status().is_success() {
    //                     anyhow::bail!(
    //                         "GET {} HTTP/1.1 -> {:?} {}",
    //                         &*discovery_rest_url,
    //                         rest_description.version(),
    //                         rest_description.status()
    //                     );
    //                 }

    //                 let rest_description = rest_description
    //                     .json::<crate::models::rest_description::RestDescription>()
    //                     .await
    //                     .with_context(|| format!("{}: decoding response body", crate_name))?;

    //                 {
    //                     let discovery_json_path = crate_path.join("discovery.json");

    //                     log::debug!("{}: writing {:?}", crate_name, discovery_json_path);
    //                     tokio::fs::OpenOptions::new()
    //                         .create(true)
    //                         .truncate(true)
    //                         .write(true)
    //                         .open(&*discovery_json_path)
    //                         .await?
    //                         .write_all(&*serde_json::to_vec_pretty(&rest_description)?)
    //                         .await?;
    //                 }

    //                 {
    //                     let crate_name_long = format!("google-api-rust-client-{}", crate_name);

    //                     let crate_description =
    //                         rest_description.description.unwrap_or(String::new());

    //                     let crate_version = format!(
    //                         "{}+{}",
    //                         env!("CARGO_PKG_VERSION"),
    //                         &*rest_description.revision
    //                     );

    //                     let cargo_toml_path = crate_path.join("Cargo.toml");

    //                     log::debug!("{}: writing {:?}", crate_name, cargo_toml_path);
    //                     tokio::fs::OpenOptions::new()
    //                         .create(true)
    //                         .truncate(true)
    //                         .write(true)
    //                         .open(&*cargo_toml_path)
    //                         .await?
    //                         .write_all(&*toml::to_vec(&toml::toml! {
    //                             [package]
    //                             name = crate_name_long
    //                             description = crate_description
    //                             version = crate_version
    //                             authors = ["Ezra Celli <ezra@ezracelli.com>"]
    //                             edition = "2018"

    //                             [dependencies]
    //                             serde = { version = "1", features = ["derive"] }
    //                             serde_json = "1"
    //                             serde_with = "1"
    //                         })?)
    //                         .await?;
    //                 }

    //                 {
    //                     let lib_rs_path = crate_path.join("src").join("lib.rs");
    //                     tokio::fs::create_dir_all(lib_rs_path.parent().unwrap()).await?;

    //                     let mut lib_rs = tokio::fs::OpenOptions::new()
    //                         .create(true)
    //                         .truncate(true)
    //                         .write(true)
    //                         .open(&*lib_rs_path)
    //                         .await?;

    //                     if let Some(schemas) = rest_description.schemas {
    //                         let tokens = schemas
    //                             .iter()
    //                             .map(|(schema_name, schema)| {
    //                                 let schema_doc =
    //                                     if let Some(ref description) = schema.description {
    //                                         quote::quote!(#[doc = #description])
    //                                     } else {
    //                                         quote::quote!()
    //                                     };

    //                                 let schema_ident = quote::format_ident!("{}", schema_name);

    //                                 let schema_fields = if let Some(ref properties) =
    //                                     schema.properties
    //                                 {
    //                                     properties
    //                                         .iter()
    //                                         .map(|(property_name, property)| {
    //                                             let property_doc = if let Some(ref description) =
    //                                                 property.description
    //                                             {
    //                                                 quote::quote!(#[doc = #description])
    //                                             } else {
    //                                                 quote::quote!()
    //                                             };

    //                                             lazy_static::lazy_static! {
    //                                                 static ref RE_RESERVED: regex::Regex =
    //                                                     regex::Regex::new(r"(?x)^(
    //                                                         as|
    //                                                         async|
    //                                                         await|
    //                                                         break|
    //                                                         const|
    //                                                         continue|
    //                                                         crate|
    //                                                         dyn|
    //                                                         else|
    //                                                         enum|
    //                                                         extern|
    //                                                         false|
    //                                                         final|
    //                                                         fn|
    //                                                         for|
    //                                                         if|
    //                                                         impl|
    //                                                         in|
    //                                                         let|
    //                                                         loop|
    //                                                         macro|
    //                                                         match|
    //                                                         mod|
    //                                                         move|
    //                                                         mut|
    //                                                         pub|
    //                                                         ref|
    //                                                         return|
    //                                                         self|
    //                                                         static|
    //                                                         struct|
    //                                                         super|
    //                                                         trait|
    //                                                         true|
    //                                                         type|
    //                                                         union|
    //                                                         unsafe|
    //                                                         use|
    //                                                         where|
    //                                                         while
    //                                                     )$").unwrap();
    //                                             }

    //                                             let property_ident =
    //                                                 change_case::snake_case(&*property_name);
    //                                             let property_ident = RE_RESERVED
    //                                                 .replace_all(&*property_ident, "_${1}");

    //                                             let serde_rename = if property_name
    //                                                 != &change_case::camel_case(&*property_ident)
    //                                                 || &*property_ident
    //                                                     != &change_case::snake_case(&*property_name)
    //                                             {
    //                                                 quote::quote!(#[serde(rename = #property_name)])
    //                                             } else {
    //                                                 quote::quote!()
    //                                             };

    //                                             let property_ident =
    //                                                 quote::format_ident!("{}", property_ident);

    //                                             let property_ty = util::property_ty(property)?;
    //                                             let property_ty = if property.required == Some(true)
    //                                             {
    //                                                 property_ty
    //                                             } else {
    //                                                 quote::quote!(Option<#property_ty>)
    //                                             };

    //                                             Ok::<_, anyhow::Error>(quote::quote! {
    //                                                 #property_doc
    //                                                 #serde_rename
    //                                                 pub #property_ident: #property_ty
    //                                             })
    //                                         })
    //                                         .collect::<Result<Vec<_>, _>>()
    //                                         .with_context(|| {
    //                                             format!(
    //                                                 "error when constructing ast for {}::{}",
    //                                                 crate_name, schema_name
    //                                             )
    //                                         })?
    //                                 } else {
    //                                     Vec::new()
    //                                 };

    //                                 Ok::<_, anyhow::Error>(quote::quote! {
    //                                     #[serde_with::skip_serializing_none]
    //                                     #[derive(Debug, serde::Serialize, serde::Deserialize)]
    //                                     #schema_doc
    //                                     #[serde(deny_unknown_fields)]
    //                                     #[serde(rename_all = "camelCase")]
    //                                     pub struct #schema_ident {
    //                                         #(#schema_fields),*
    //                                     }
    //                                 })
    //                             })
    //                             .collect::<Result<Vec<_>, _>>()?;

    //                         let code = quote::quote!(#(#tokens)*);
    //                         if code.is_empty() {
    //                             anyhow::bail!("{}: no code generated", crate_name);
    //                         }

    //                         log::debug!("{}: writing {:?}", crate_name, lib_rs_path);
    //                         lib_rs.write_all(code.to_string().as_bytes()).await?;

    //                         let rustfmt_output = tokio::process::Command::new(
    //                             std::env::var_os("RUSTFMT").unwrap_or("rustfmt".into()),
    //                         )
    //                         .args(&["--edition", "2018"])
    //                         .arg(lib_rs_path)
    //                         .env_remove("RUST_LOG")
    //                         .output()
    //                         .await?;

    //                         if rustfmt_output.stderr.len() > 0 {
    //                             anyhow::bail!(
    //                                 "{}: rustfmt said:\n{}",
    //                                 crate_name,
    //                                 String::from_utf8(rustfmt_output.stderr)?
    //                             );
    //                         }
    //                     }
    //                 }

    //                 Ok::<_, anyhow::Error>(())
    //             } else {
    //                 anyhow::bail!(
    //                     "DirectoryItem.discovery_rest_url for {:?} was null or not provided",
    //                     item.id
    //                 );
    //             }
    //         }
    //     }))
    //     .await;

    //     let errors = results
    //         .into_iter()
    //         .filter(Result::is_err)
    //         .map(Result::unwrap_err)
    //         .collect::<Vec<_>>();

    //     for error in errors.iter() {
    //         log::warn!("{:?}", error)
    //     }

    //     if errors.len() > 0 {
    //         std::process::exit(1);
    //     }
    // } else {
    //     log::error!("DirectoryList.items was null or not provided");
    //     std::process::exit(1);
    // }

    // Ok(())
}
