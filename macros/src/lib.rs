mod dom;
mod view;

use proc_macro as pm1;

use quote::quote;

#[proc_macro]
pub fn view(input: pm1::TokenStream) -> pm1::TokenStream {
    let view = view::parse_view(input.into());
    quote! { #view }.into()
}

#[proc_macro]
pub fn routes(_input: pm1::TokenStream) -> pm1::TokenStream {
    todo!()
}

#[proc_macro]
pub fn document(input: pm1::TokenStream) -> pm1::TokenStream {
    let document = dom::parse_document(input.into());
    quote! { #document }.into()
}
