use proc_macro::TokenStream;
use quote::quote;

// YewMessage -> ComponentMessage
// YewModel -> ComponentModel
// YewDebug -> ComponentDebug

#[proc_macro_derive(YewMessage)]
pub fn derive_yew_message(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl YewMessage for #name {}
    };
    gen.into()
}

#[proc_macro_derive(YewModel)]
pub fn derive_yew_model(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl YewModel for #name {}
    };
    gen.into()
}

#[proc_macro_derive(YewDebug)]
pub fn derive_yew_debug(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl YewDebug for #name {}
    };
    gen.into()
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
