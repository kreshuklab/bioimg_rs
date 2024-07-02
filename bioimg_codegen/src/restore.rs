use quote::{quote, quote_spanned, format_ident};
use syn::spanned::Spanned;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub fn do_derive_restore(input: TokenStream) -> syn::Result<TokenStream>{
    // Parse the input tokens into a syntax tree.
    let input = syn::parse::<syn::ItemStruct>(input)?;
    let struct_name = &input.ident;
    let raw_data_struct_name = format_ident!("{}RawData", struct_name);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let raw_data_field_initializers: Vec<TokenStream2> = input.fields.iter()
        .enumerate()
        .filter_map(|(field_idx, field)| {
            let ident = field.ident.as_ref().map(|id| quote!(#id)).unwrap_or(quote!(#field_idx));
            let ident_span = ident.span();
            if field.attrs.iter().find(|attr| attr.path().is_ident("skip_restore")).is_some(){
                return None
            }
            Some(quote_spanned! {ident_span=>
                #ident: crate::widgets::Restore::dump(&self.#ident),
            })
        })
        .collect();

    let restore_statements: Vec<TokenStream2> = input.fields.iter()
        .enumerate()
        .map(|(field_idx, field)|{
            let ident = field.ident.as_ref().map(|id| quote!(#id)).unwrap_or(quote!(#field_idx));
            let span = ident.span();
            let ty_span = field.ty.span();
            if field.attrs.iter().find(|attr| attr.path().is_ident("skip_restore")).is_some(){
                quote_spanned! {ty_span=>
                    self.#ident = std::default::Default::default();
                }
            } else {
                quote_spanned! {span=>
                    crate::widgets::Restore::restore(&mut self.#ident, raw_data.#ident);
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics crate::widgets::Restore for #struct_name #ty_generics #where_clause {
            type RawData = crate::project_data::#raw_data_struct_name;
            fn dump(&self) -> Self::RawData #ty_generics{
                crate::project_data::#raw_data_struct_name{
                    #(#raw_data_field_initializers)*
                }
            }
            fn restore(&mut self, raw_data: Self::RawData){
                #(#restore_statements)*
            }
        }
    };

    Ok(proc_macro::TokenStream::from(expanded))
}
