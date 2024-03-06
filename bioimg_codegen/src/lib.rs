#![allow(incomplete_features)]
// #![feature(proc_macro_diagnostic, adt_const_params)]
#![allow(non_snake_case)]

use proc_macro::TokenStream;

mod modelrecord;
mod syn_extensions;

////////////////////////////////////////////

#[proc_macro_derive(BinRecord, attributes(binrecord))]
pub fn derive_binrecord(input: TokenStream) -> TokenStream {
    match modelrecord::do_derive_ModelRecord(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}
