/// This trait is used to parse a configuration for the application.
///
/// # See Also
///
/// - [`Global`](crate::Global)
pub trait ConfigParser: Sized {
    fn parse() -> impl std::future::Future<Output = anyhow::Result<Self>>;
}

impl ConfigParser for () {
    #[inline(always)]
    fn parse() -> impl std::future::Future<Output = anyhow::Result<Self>> {
        std::future::ready(Ok(()))
    }
}

pub struct EmptyConfig;

impl ConfigParser for EmptyConfig {
    #[inline(always)]
    fn parse() -> impl std::future::Future<Output = anyhow::Result<Self>> {
        std::future::ready(Ok(EmptyConfig))
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use super::{ConfigParser, EmptyConfig};

    #[tokio::test]
    async fn unit_config() {
        assert!(matches!(<()>::parse().await, Ok(())));
    }

    #[tokio::test]
    async fn empty_config() {
        assert!(matches!(EmptyConfig::parse().await, Ok(EmptyConfig)));
    }
}
