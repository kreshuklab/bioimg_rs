use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
// use quote::{format_ident, quote, quote_spanned};
// use syn::spanned::Spanned;

// use crate::syn_extensions;

pub fn do_derive_str_marker(_input: TokenStream) -> syn::Result<TokenStream> {
    // Parse the input tokens into a syntax tree.
    // let input = syn::parse::<syn::ItemStruct>(input)?;
    // let struct_name = &input.ident;
    // let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    unimplemented!("Still need to parse an attribute with the marker name");

    // let expanded = quote! {
    //     impl #impl_generics bioimg_spec::rdf::StrMarker for #struct_name {
    //         const NAME: &'static str = #( #field_sizes + )* 0;
    //     }
    // };

    // Ok(proc_macro::TokenStream::from(expanded))
}
