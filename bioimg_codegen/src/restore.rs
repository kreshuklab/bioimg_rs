use quote::{quote, quote_spanned, format_ident};
use syn::spanned::Spanned;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub const RESTORE_ON_UPDATE: &str = "restore_on_update";
pub const RESTORE_DEFAULT: &str = "restore_default";

enum FieldRestoreMode{
    ViaDefault,
    OnUpdate(syn::Attribute),
    FromRaw,
}

impl FieldRestoreMode{
    pub fn try_from_attrs<'a>(attrs: impl IntoIterator<Item=&'a syn::Attribute>) -> syn::Result<Self>{
        let mut out = Self::FromRaw;
        for attr in attrs.into_iter(){
            let old_out = if attr.path().is_ident(RESTORE_DEFAULT){
                std::mem::replace(&mut out, Self::ViaDefault)
            } else if attr.path().is_ident(RESTORE_ON_UPDATE){
                std::mem::replace(&mut out, Self::OnUpdate(attr.clone()))
            } else {
                Self::FromRaw
            };
            if ! matches!(old_out, Self::FromRaw){
                return Err(syn::Error::new(attr.span(), "Conflicting restore strategy"))
            }
        }
        Ok(out)
    }

    pub fn skips_dump(&self) -> bool{
        !matches!(self, Self::FromRaw)
    }
}

pub fn do_derive_restore(input: TokenStream) -> syn::Result<TokenStream>{
    // Parse the input tokens into a syntax tree.
    let input = syn::parse::<syn::ItemStruct>(input)?;
    let struct_name = &input.ident;
    let raw_data_struct_name = format_ident!("{}RawData", struct_name);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut raw_data_field_initializers = Vec::<TokenStream2>::new();
    for (field_idx, field) in input.fields.iter().enumerate(){
        let ident = field.ident.as_ref().map(|id| quote!(#id)).unwrap_or(quote!(#field_idx));
        let ident_span = ident.span();
        if FieldRestoreMode::try_from_attrs(field.attrs.iter())?.skips_dump(){
            continue;
        }
        raw_data_field_initializers.push(quote_spanned! {ident_span=>
            #ident: crate::widgets::Restore::dump(&self.#ident),
        });
    }

    let mut restore_statements = Vec::<TokenStream2>::new();
    let mut update_trigger: Option<syn::Attribute> = None;
    for (field_idx, field) in input.fields.iter().enumerate(){
        let ident = field.ident.as_ref().map(|id| quote!(#id)).unwrap_or(quote!(#field_idx));
        let span = ident.span();
        let ty_span = field.ty.span();

        let statement = match FieldRestoreMode::try_from_attrs(field.attrs.iter())?{
            FieldRestoreMode::ViaDefault => quote_spanned! {ty_span=>
                self.#ident = std::default::Default::default();
            },
            FieldRestoreMode::OnUpdate(attr) => {
                update_trigger = Some(attr);
                quote!{}
            },
            FieldRestoreMode::FromRaw => quote_spanned! {span=>
                crate::widgets::Restore::restore(&mut self.#ident, raw_data.#ident);
            }
        };
        restore_statements.push(statement);
    }

    if let Some(attr) = update_trigger{
        let span = attr.span(); 
        restore_statements.push(quote_spanned! {span=>
            self.update();
        })
    }

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
