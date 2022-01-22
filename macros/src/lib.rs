extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BindingTypeView)]
pub fn derive_binding_type_view(_item: TokenStream) -> TokenStream {
    let struct_name = &parse_macro_input!(_item as DeriveInput).ident;
    let code = quote::quote! {
        impl BindingTypeView for #struct_name {}
    };
    TokenStream::from(code)
}

