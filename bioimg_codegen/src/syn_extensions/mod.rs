use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;

pub trait AttributeExt {
    fn is_varlen(&self) -> syn::Result<bool>;
}

impl AttributeExt for syn::Attribute {
    fn is_varlen(&self) -> syn::Result<bool> {
        let syn::Meta::List(syn::MetaList { path, tokens, .. }) = &self.meta else { return Ok(false) };
        if quote!(#path).to_string() != "binrecord" {
            return Ok(false);
        }
        if quote!(#tokens).to_string() != "varlen" {
            return Err(syn::Error::new_spanned(self.clone(), "Expected 'varlen' value"));
        }
        Ok(true)
    }
}

pub struct FieldParseParams<'a> {
    pub offset: &'a syn::Ident,
    pub struct_name: &'a syn::Ident,
}

pub trait FieldExt {
    fn is_varlen(&self) -> syn::Result<bool>;
    fn generate_try_parse(&self, params: FieldParseParams) -> syn::Result<TokenStream2>;
}

impl FieldExt for syn::Field {
    fn is_varlen(&self) -> syn::Result<bool> {
        for attr in &self.attrs {
            if attr.is_varlen()? {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    fn generate_try_parse(&self, params: FieldParseParams) -> syn::Result<TokenStream2> {
        let FieldParseParams { offset, struct_name } = params;

        let field_span = self.span();
        let Some(field_name) = &self.ident else { return Err(syn::Error::new(self.span(), "Field must me named")) };
        let record_name_lit = struct_name.to_lit_str();
        let field_name_lit = field_name.to_lit_str();

        if self.is_varlen()? {
            let length_field_ident = format_ident!("{field_name}_length");
            Ok(quote_spanned! {field_span=>
                let mut #field_name: Vec<u8> = vec![0; #length_field_ident as usize];
                stream.read_exact(&mut $varlen_field_name).map_err(|io_err| RecordParsingError::ParsingError{
                    record_name: #record_name_lit,
                    field_name: #field_name_lit,
                    offset,
                    source: Box::new(RecordParsingError::from(io_err)),
                })?;
                #offset += #length_field_ident as usize;
            })
        } else {
            let field_type = &self.ty;
            Ok(quote_spanned! { field_span=>
                let #field_name = match <#field_type as vise::BinRecord>::try_parse(stream){
                    Ok(val) => val,
                    Err(source) => return Err(RecordParsingError::ParsingError{
                        record_name: #record_name_lit,
                        field_name: #field_name_lit,
                        offset,
                        source: Box::new(source),
                    })
                };
                #offset += <#field_type as vise::BinRecord>::PACKED_SIZE;
            })
        }
    }
}

pub trait IdentExt {
    fn to_lit_str(&self) -> syn::LitStr;
}

impl IdentExt for syn::Ident {
    fn to_lit_str(&self) -> syn::LitStr {
        syn::LitStr::new(&quote!(#self).to_string(), self.span())
    }
}
