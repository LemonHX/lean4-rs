use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, ExprLit, Fields};
extern crate proc_macro;
use proc_macro::TokenStream;

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

#[proc_macro_derive(Lean4Inductive)]
pub fn lean4_inductive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name and variants of the enum
    let name = &input.ident;
    let union_name = format_ident!("Lean4Union{}", name);

    let variants = match input.data {
        Data::Enum(ref data) => &data.variants,
        _ => panic!("Lean4Inductive can only be derived for enumerations"),
    };

    // Generate the fields and types for the union
    let mut union_fields = quote!();
    let mut match_cases = quote!();
    let mut match_cases2 = quote!();

    for (index, variant) in variants.iter().enumerate() {
        let index = index as u32;
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields) => {

                union_fields = quote! {
                    #union_fields
                    #variant_ident : #fields
                };

                let mut ctor_gets = quote!();
                {
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        ctor_gets = quote! {
                            #ctor_gets
                            lean_ctor_get(obj.into(), #i as u32).into(),
                        }
                    }
                    ctor_gets = quote! {
                        ( #ctor_gets )
                    };
                }
                match_cases = quote! {
                    #match_cases
                    #index => #union_name {
                        #variant_ident: #ctor_gets
                    },
                };

                let mut ctor_gets2 = quote!();
                {
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        let i = ExprLit { attrs: vec![], lit: syn::Lit::Int(syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site())) };
                        // i to literal num
                        ctor_gets2 = quote! {
                            #ctor_gets2
                            obj.data.#variant_ident.#i,
                        }
                    }
                    ctor_gets2 = quote! {
                        ( #ctor_gets2 )
                    };
                }

                match_cases2 = quote! {
                    #match_cases2
                    #index => #name :: #variant_ident #ctor_gets2,
                }
            },
            Fields::Unit => {
                union_fields = quote! {
                    #union_fields
                    #variant_ident : (),
                };
                match_cases = quote! {
                    #match_cases
                    #index => #union_name {
                        #variant_ident: ()
                    },
                };
                match_cases2 = quote! {
                    #match_cases2
                    #index => #name :: #variant_ident,
                }
            },
            _ => panic!("Lean4Inductive can only be derived for enumerations with unnamed fields or unit variants"),
        };
    }
    match_cases = quote! {
        #match_cases
        _ => unreachable!(),
    };
    match_cases2 = quote! {
        #match_cases2
        _ => unreachable!(),
    };

    let struct_name = format_ident!("Lean4UStruct{}", name);

    let output = quote! {
        #[allow(non_snake_case)]
        impl From<Lean4Obj> for #name {
            fn from(obj: Lean4Obj) -> Self {
                union #union_name {
                    #union_fields,
                }

                #[repr(packed)]
                struct #struct_name {
                    tag: u32,
                    data: #union_name,
                }

                impl #struct_name {
                    pub fn new(obj: Lean4Obj) -> Self {
                        unsafe {
                            let tag = lean_obj_tag(obj.0);
                            let data = match tag {
                                #match_cases
                            };
                            #struct_name { tag, data }
                        }
                    }
                }

                unsafe {
                    let obj = #struct_name::new(obj);
                    match obj.tag {
                        #match_cases2
                    }
                }
            }
        }
    };
    output.into()
}
