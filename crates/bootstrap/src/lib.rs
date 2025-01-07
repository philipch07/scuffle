#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

pub mod config;
pub mod global;
pub mod service;

pub use config::{ConfigParser, EmptyConfig};
pub use global::Global;
pub use scuffle_bootstrap_derive::main;
pub use service::Service;

#[doc(hidden)]
pub mod prelude {
    pub use {anyhow, futures, scuffle_context, tokio};
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    #[test]
    fn main_test() {
        insta::assert_snapshot!(postcompile::compile! {
            use std::sync::Arc;

            use scuffle_bootstrap::main;

            struct TestGlobal;

            impl scuffle_signal::SignalConfig for TestGlobal {}

            impl scuffle_bootstrap::global::GlobalWithoutConfig for TestGlobal {
                async fn init() -> anyhow::Result<Arc<Self>> {
                    Ok(Arc::new(Self))
                }
            }

            main! {
                TestGlobal {
                    scuffle_signal::SignalSvc,
                }
            }
        });
    }

    #[test]
    fn main_test_custom_service() {
        insta::assert_snapshot!(postcompile::compile! {
            use std::sync::Arc;

            use scuffle_bootstrap::main;

            struct TestGlobal;

            impl scuffle_signal::SignalConfig for TestGlobal {}

            impl scuffle_bootstrap::global::GlobalWithoutConfig for TestGlobal {
                async fn init() -> anyhow::Result<Arc<Self>> {
                    Ok(Arc::new(Self))
                }
            }

            struct MySvc;

            impl scuffle_bootstrap::service::Service<TestGlobal> for MySvc {
                async fn run(self, _: Arc<TestGlobal>, _: scuffle_context::Context) -> anyhow::Result<()> {
                    println!("running");
                    Ok(())
                }
            }

            main! {
                TestGlobal {
                    scuffle_signal::SignalSvc,
                    MySvc,
                }
            }
        });
    }
}
