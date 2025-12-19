//! Macro definitions for kernel module functions.
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Attribute macro to mark the initialization function of a kernel module. It
/// places the function in the `.text.init` section.
/// # Example:
/// ```ignore
/// #[init_fn]
/// fn init() -> i32 { ... }
/// ```
#[proc_macro_attribute]
pub fn init_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as syn::ItemFn);

    quote! {
        #[unsafe(link_section = ".text.init")]
        #func
    }
    .into()
}

/// Attribute macro to mark the cleanup function of a kernel module. It places
/// the function in the `.text.exit` section.
/// # Example:
/// ```ignore
/// #[exit_fn]
/// fn cleanup() { ... }
/// ```
#[proc_macro_attribute]
pub fn exit_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as syn::ItemFn);
    quote! {
        #[unsafe(link_section = ".text.exit")]
        #func
    }
    .into()
}
