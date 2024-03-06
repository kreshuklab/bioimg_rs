use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;

use crate::syn_extensions::{FieldExt, FieldParseParams};

pub fn do_derive_ModelRecord(input: TokenStream) -> syn::Result<TokenStream> {
    // Parse the input tokens into a syntax tree.
    let input = syn::parse::<syn::ItemStruct>(input)?;
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let offset_ident = format_ident!("offset");

    let fields_parsing_steps = input
        .fields
        .iter()
        .map(|field| field.generate_try_parse(FieldParseParams { offset: &offset_ident, struct_name: &struct_name }))
        .collect::<syn::Result<Vec<_>>>()?;

    let mut field_sizes: Vec<TokenStream2> = vec![];
    for field in input.fields.iter() {
        if field.is_varlen()? {
            continue;
        }
        let field_type = &field.ty;
        let field_span = field.span();
        field_sizes.push(quote_spanned!(field_span=>
            <#field_type as vise::BinRecord>::PACKED_SIZE
        ));
    }

    let instantiate_self_initializers: Vec<&syn::Ident> = input
        .fields
        .iter()
        .map(|field| {
            let Some(ident) = &field.ident else { return Err(syn::Error::new(field.span(), "Expected named field")) };
            Ok(ident)
        })
        .collect::<syn::Result<Vec<&syn::Ident>>>()?;

    let expanded = quote! {
        impl #impl_generics vise::BinRecord for #struct_name #ty_generics #where_clause {
            const PACKED_SIZE: usize = #( #field_sizes + )* 0;

            fn try_parse<S: std::io::Read>(stream: &mut S) -> Result<Self, RecordParsingError>{
                let mut #offset_ident: usize = 0;
                #( #fields_parsing_steps )*
                Ok(Self{
                    #( #instantiate_self_initializers ),*
                })
            }
        }
    };

    Ok(proc_macro::TokenStream::from(expanded))
}
