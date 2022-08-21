use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn chrome_native_task(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(tag = "task", content = "message")]
        #input
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn chrome_native_data(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input
    };

    expanded.into()
}

fn generator(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        if ast.attrs.len() != 0 {
            let default_gen: syn::LitBool = ast.attrs[0].parse_args().unwrap();
            let generate = default_gen.value();
            if generate {
                let all_option = named.iter().all(|f| {
                    if let syn::Type::Path(ref p) = f.ty {
                        p.path.segments.len() == 1 && p.path.segments[0].ident == "Option"
                    } else {
                        false
                    }
                });
                if all_option {
                    let options_value = named.iter().map(|f| {
                        let name = &f.ident;
                        quote! {
                            #name: None
                        }
                    });
                    return quote! {
                        impl #name {
                            fn create_self() -> Self {
                                Self {
                                    #(#options_value,)*
                                }
                            }
                        }
                    }
                    .into();
                }
            }
        }
    }
    quote! {}.into()
}

#[proc_macro_derive(Plugin, attributes(default_gen))]
pub fn plugin_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generated = &generator(&ast);

    let expanded = quote! {
        #generated

        #[no_mangle]
        pub extern fn get_plugin() -> *mut dyn chrome_native::Plugin {
            Box::into_raw(Box::new(#name::create_self()))
        }
    };

    expanded.into()
}
