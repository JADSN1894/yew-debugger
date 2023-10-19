use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(YewModel)]
pub fn derive_yew_model(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        use yew_debugger::YewComponentModel;
        impl YewComponentModel for #name {}
    };
    gen.into()
}

#[proc_macro_derive(YewMessage)]
pub fn derive_yew_message(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        use yew_debugger::YewComponentMessage;
        impl YewComponentMessage for #name {}
    };
    gen.into()
}
