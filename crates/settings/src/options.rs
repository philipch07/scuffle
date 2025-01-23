#[derive(Debug, Clone)]
pub struct Options {
    /// The CLI options
    #[cfg(feature = "cli")]
    pub cli: Option<Cli>,
    /// The default config file name (loaded if no other files are specified)
    pub default_config_file: Option<&'static str>,
    /// Environment variables prefix
    pub env_prefix: Option<&'static str>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            #[cfg(feature = "cli")]
            cli: None,
            default_config_file: Some("config"),
            env_prefix: Some("APP"),
        }
    }
}

/// A struct used to define how the CLI should be generated
#[derive(Debug, Clone)]
pub struct Cli {
    /// The name of the program
    pub name: &'static str,

    /// The version of the program
    pub version: &'static str,

    /// The about of the program
    pub about: &'static str,

    /// The author of the program
    pub author: &'static str,

    /// The arguments to add to the CLI
    pub argv: Vec<String>,
}

/// A macro to create a CLI struct
/// This macro will automatically set the name, version, about, and author from
/// the environment variables at compile time
#[macro_export]
macro_rules! cli {
    () => {
        $crate::cli!(std::env::args().collect())
    };
    ($args:expr) => {
        $crate::Cli {
            name: env!("CARGO_BIN_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            about: env!("CARGO_PKG_DESCRIPTION"),
            author: env!("CARGO_PKG_AUTHORS"),
            argv: $args,
        }
    };
}
