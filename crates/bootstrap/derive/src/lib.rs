#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

use proc_macro::TokenStream;

mod main_impl;
// mod service_impl;

// #[proc_macro_attribute]
// pub fn service(args: TokenStream, input: TokenStream) -> TokenStream {
// 	handle_error(service_impl::impl_service(args.into(), input.into()))
// }

#[proc_macro]
pub fn main(input: TokenStream) -> TokenStream {
    handle_error(main_impl::impl_main(input.into()))
}

fn handle_error(input: Result<proc_macro2::TokenStream, syn::Error>) -> TokenStream {
    match input {
        Ok(value) => value.into(),
        Err(err) => err.to_compile_error().into(),
    }
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
}
