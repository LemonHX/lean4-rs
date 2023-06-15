use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
extern crate proc_macro;

// write a derive macro

#[proc_macro_derive(Lean4)]
pub fn lean4_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let d = proc_macro::TokenStream::from(input.clone());
    let DeriveInput { ident, .. } = parse_macro_input!(d);
    let varname = format_ident!("lean4_external_class_{}", ident);
    let lean4object_impl = quote! {
        #[no_mangle]
        pub static mut #varname: *mut lean_external_class = std::ptr::null_mut();
        impl Lean4Object for #ident {
            #[inline(always)]
            fn get_registed_class() -> &'static mut *mut lean_external_class {
                unsafe { &mut #varname }
            }
        }
    };
    // combine input tokens with new tokens
    proc_macro::TokenStream::from(lean4object_impl)
}
