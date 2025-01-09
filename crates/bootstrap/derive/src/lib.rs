use proc_macro::TokenStream;

mod main_impl;
// mod service_impl;

// #[proc_macro_attribute]
// pub fn service(args: TokenStream, input: TokenStream) -> TokenStream {
// 	handle_error(service_impl::impl_service(args.into(), input.into()))
// }

/// This macro is used to generate the main function for a given global type
/// and service types. It will run all the services in parallel and wait for
/// them to finish before exiting.
///
/// # Example
///
/// ```rust,ignore
/// # // We cant test this example because it depends on the parent crate and we don't want to introduce a cyclic dependency
/// scuffle_bootstrap::main! {
///     MyGlobal {
///         scuffle_signal::SignalSvc,
///         MyService,
///     }
/// }
/// ```
///
/// # See Also
///
/// - `scuffle_bootstrap::Service`
/// - `scuffle_bootstrap::Global`
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
