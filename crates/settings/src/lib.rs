//! A crate designed to provide a simple interface to load and manage settings for Scuffle.
//!
//! ## Why do we need this?
//!
//! TODO(troy): Add more details about why we need this.
//!
//! ## Status
//!
//! This crate is currently under development and is not yet stable.
//!
//! Unit tests are not yet fully implemented. Use at your own risk.
//!
//! ## License
//!
//! This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
//! You can choose between one of them if you use this work.
//!
//! `SPDX-License-Identifier: MIT OR Apache-2.0`
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

use std::borrow::Cow;
use std::path::Path;

use config::FileStoredFormat;

mod options;

pub use options::*;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[cfg(feature = "cli")]
    #[error(transparent)]
    Clap(#[from] clap::Error),
}

#[derive(Debug, Clone, Copy)]
struct FormatWrapper;

#[cfg(not(feature = "templates"))]
fn template_text<'a>(
    text: &'a str,
    _: &config::FileFormat,
) -> Result<Cow<'a, str>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Cow::Borrowed(text))
}

#[cfg(feature = "templates")]
fn template_text<'a>(
    text: &'a str,
    _: &config::FileFormat,
) -> Result<Cow<'a, str>, Box<dyn std::error::Error + Send + Sync>> {
    use minijinja::syntax::SyntaxConfig;

    let mut env = minijinja::Environment::new();

    env.add_global("env", std::env::vars().collect::<std::collections::HashMap<_, _>>());
    env.set_syntax(
        SyntaxConfig::builder()
            .block_delimiters("{%", "%}")
            .variable_delimiters("${{", "}}")
            .comment_delimiters("{#", "#}")
            .build()
            .unwrap(),
    );

    Ok(Cow::Owned(env.template_from_str(text).unwrap().render(())?))
}

impl config::Format for FormatWrapper {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<config::Map<String, config::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let uri_ext = uri.and_then(|s| Path::new(s.as_str()).extension()).and_then(|s| s.to_str());

        let mut formats: Vec<config::FileFormat> = vec![
            #[cfg(feature = "toml")]
            config::FileFormat::Toml,
            #[cfg(feature = "json")]
            config::FileFormat::Json,
            #[cfg(feature = "yaml")]
            config::FileFormat::Yaml,
            #[cfg(feature = "json5")]
            config::FileFormat::Json5,
            #[cfg(feature = "ini")]
            config::FileFormat::Ini,
            #[cfg(feature = "ron")]
            config::FileFormat::Ron,
        ];

        if let Some(uri_ext) = uri_ext {
            formats.sort_by_key(|f| if f.file_extensions().contains(&uri_ext) { 0 } else { 1 });
        }

        for format in formats {
            if let Ok(map) = format.parse(uri, template_text(text, &format)?.as_ref()) {
                return Ok(map);
            }
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("No supported format found for file: {:?}", uri),
        )))
    }
}

impl config::FileStoredFormat for FormatWrapper {
    fn file_extensions(&self) -> &'static [&'static str] {
        &[
            #[cfg(feature = "toml")]
            "toml",
            #[cfg(feature = "json")]
            "json",
            #[cfg(feature = "yaml")]
            "yaml",
            #[cfg(feature = "yaml")]
            "yml",
            #[cfg(feature = "json5")]
            "json5",
            #[cfg(feature = "ini")]
            "ini",
            #[cfg(feature = "ron")]
            "ron",
        ]
    }
}

pub fn parse_settings<T: serde::de::DeserializeOwned>(options: Options) -> Result<T, ConfigError> {
    let mut config = config::Config::builder();

    #[allow(unused_mut)]
    let mut added_files = false;

    #[cfg(feature = "cli")]
    if let Some(cli) = options.cli {
        let command = clap::Command::new(cli.name)
            .version(cli.version)
            .about(cli.about)
            .author(cli.author)
            .bin_name(cli.name)
            .arg(
                clap::Arg::new("config")
                    .short('c')
                    .long("config")
                    .value_name("FILE")
                    .help("Path to configuration file(s)")
                    .action(clap::ArgAction::Append),
            )
            .arg(
                clap::Arg::new("overrides")
                    .long("override")
                    .short('o')
                    .alias("set")
                    .help("Provide an override for a configuration value, in the format KEY=VALUE")
                    .action(clap::ArgAction::Append),
            );

        let matches = command.get_matches_from(cli.argv);

        if let Some(config_files) = matches.get_many::<String>("config") {
            for path in config_files {
                config = config.add_source(config::File::new(path, FormatWrapper));
                added_files = true;
            }
        }

        if let Some(overrides) = matches.get_many::<String>("overrides") {
            for ov in overrides {
                let (key, value) = ov.split_once('=').ok_or_else(|| {
                    clap::Error::raw(
                        clap::error::ErrorKind::InvalidValue,
                        "Override must be in the format KEY=VALUE",
                    )
                })?;

                config = config.set_override(key, value)?;
            }
        }
    }

    if !added_files {
        if let Some(default_config_file) = options.default_config_file {
            config = config.add_source(config::File::new(default_config_file, FormatWrapper).required(false));
        }
    }

    if let Some(env_prefix) = options.env_prefix {
        config = config.add_source(config::Environment::with_prefix(env_prefix));
    }

    Ok(config.build()?.try_deserialize()?)
}

#[doc(hidden)]
#[cfg(feature = "bootstrap")]
pub mod macros {
    pub use {anyhow, scuffle_bootstrap};
}

/// A macro to create a config parser from a CLI struct
/// This macro will automatically parse the CLI struct into the given type
/// using the `scuffle-settings` crate
#[cfg(feature = "bootstrap")]
#[macro_export]
macro_rules! bootstrap {
    ($ty:ty) => {
        impl $crate::macros::scuffle_bootstrap::config::ConfigParser for $ty {
            async fn parse() -> $crate::macros::anyhow::Result<Self> {
                $crate::macros::anyhow::Context::context(
                    $crate::parse_settings($crate::Options {
                        cli: Some($crate::cli!()),
                        ..::std::default::Default::default()
                    }),
                    "config",
                )
            }
        }
    };
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use crate::{parse_settings, Cli, Options};

    #[derive(Debug, serde::Deserialize)]
    struct TestSettings {
        key: String,
    }

    #[test]
    fn parse_empty() {
        let err = parse_settings::<TestSettings>(Options::default()).expect_err("expected error");
        assert!(matches!(err, crate::ConfigError::Config(config::ConfigError::Message(_))));
        assert_eq!(err.to_string(), "missing field `key`");
    }

    #[test]
    fn parse_cli() {
        let options = Options {
            cli: Some(Cli {
                name: "test",
                version: "0.1.0",
                about: "test",
                author: "test",
                argv: vec!["test".to_string(), "-o".to_string(), "key=value".to_string()],
            }),
            ..Default::default()
        };
        let settings: TestSettings = parse_settings(options).expect("failed to parse settings");

        assert_eq!(settings.key, "value");
    }

    #[test]
    fn cli_error() {
        let options = Options {
            cli: Some(Cli {
                name: "test",
                version: "0.1.0",
                about: "test",
                author: "test",
                argv: vec!["test".to_string(), "-o".to_string(), "error".to_string()],
            }),
            ..Default::default()
        };
        let err = parse_settings::<TestSettings>(options).expect_err("expected error");

        if let crate::ConfigError::Clap(err) = err {
            assert_eq!(err.to_string(), "error: Override must be in the format KEY=VALUE");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn parse_file() {
        let options = Options {
            cli: Some(Cli {
                name: "test",
                version: "0.1.0",
                about: "test",
                author: "test",
                argv: vec!["test".to_string(), "-c".to_string(), "assets/test.toml".to_string()],
            }),
            ..Default::default()
        };
        let settings: TestSettings = parse_settings(options).expect("failed to parse settings");

        assert_eq!(settings.key, "filevalue");
    }

    #[test]
    fn file_error() {
        let options = Options {
            cli: Some(Cli {
                name: "test",
                version: "0.1.0",
                about: "test",
                author: "test",
                argv: vec!["test".to_string(), "-c".to_string(), "assets/invalid.txt".to_string()],
            }),
            ..Default::default()
        };
        let err = parse_settings::<TestSettings>(options).expect_err("expected error");

        if let crate::ConfigError::Config(config::ConfigError::FileParse { uri: Some(uri), cause }) = err {
            assert_eq!(uri, "assets/invalid.txt");
            assert_eq!(cause.to_string(), "No supported format found for file: Some(\"assets/invalid.txt\")");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn parse_env() {
        let options = Options {
            cli: Some(Cli {
                name: "test",
                version: "0.1.0",
                about: "test",
                author: "test",
                argv: vec![],
            }),
            env_prefix: Some("SETTINGS_PARSE_ENV_TEST"),
            ..Default::default()
        };
        std::env::set_var("SETTINGS_PARSE_ENV_TEST_KEY", "envvalue");
        let settings: TestSettings = parse_settings(options).expect("failed to parse settings");

        assert_eq!(settings.key, "envvalue");
    }

    #[test]
    fn overrides() {
        let options = Options {
            cli: Some(Cli {
                name: "test",
                version: "0.1.0",
                about: "test",
                author: "test",
                argv: vec!["test".to_string(), "-o".to_string(), "key=value".to_string()],
            }),
            env_prefix: Some("SETTINGS_OVERRIDES_TEST"),
            ..Default::default()
        };
        std::env::set_var("SETTINGS_OVERRIDES_TEST_KEY", "envvalue");
        let settings: TestSettings = parse_settings(options).expect("failed to parse settings");

        assert_eq!(settings.key, "value");
    }
}
