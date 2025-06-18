use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Field, Fields, Ident, ImplItem, Item, ItemMod, Meta, parse_quote,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

use crate::common::{parse_meta_name_value, wrap_return_type};

pub(super) fn parse_implementation_args(
    args: Punctuated<Meta, Comma>,
) -> (Option<Ident>, Option<Ident>, Ident) {
    let mut trait_name = None;
    let mut impl_name = None;
    let mut result_type = None;

    for arg in args {
        match arg {
            Meta::NameValue(nv) if nv.path.is_ident("r#trait") => {
                trait_name = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("name") => {
                impl_name = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("result") => {
                result_type = parse_meta_name_value(nv);
            },
            _ => {},
        }
    }

    let trait_name = trait_name.map(|name| format_ident!("{name}"));
    let impl_name = impl_name.map(|name| format_ident!("{name}"));
    let result_type = result_type.expect("Missing result type");

    (trait_name, impl_name, format_ident!("{result_type}"))
}

pub(super) fn process_handler_implementation(
    trait_name: Ident,
    impl_name: Ident,
    result_type: Ident,
    mut input_mod: ItemMod,
) -> TokenStream {
    let attrs = &input_mod.attrs;
    let vis = &input_mod.vis;

    let items = if let Some((_, items)) = input_mod.content.take() {
        items
            .into_iter()
            .map(|item| match item {
                Item::Fn(mut func) => {
                    if wrap_return_type(&mut func.sig, &result_type) {
                        let body = func.block.clone();
                        *func.block = parse_quote! {
                            {
                                 let res = #body;
                                #[allow(unreachable_code)]
                                Ok(res)
                            }
                        };
                    }
                    let trait_name = trait_name.clone().to_string();
                    let func_name = func.sig.ident.to_string();
                    let trace_name = format!("{trait_name}::{func_name}");
                    func.attrs.push(parse_quote! { #[macros::instrument(skip_all, #trace_name)] });
                    Item::Fn(func)
                },
                _ => item,
            })
            .collect()
    } else {
        vec![]
    };

    let method_idents: Vec<_> = items
        .iter()
        .filter_map(|item| match item {
            Item::Fn(func) => Some(&func.sig.ident),
            _ => None,
        })
        .collect();

    let routes_mod = quote! {
        mod __routes {
            use super::*;

            #[derive(utoipa::OpenApi)]
            #[openapi(paths(
                #(#method_idents),*
            ))]
            pub struct OpenApi;

            #(#items)*

        }
        pub use __routes::OpenApi;
    };

    let handler_impl = quote! {
        #[derive(Clone)]
        #(#attrs)*
        #vis struct #impl_name;

        impl #trait_name for #impl_name {
            #(
                #[tracing::instrument(skip_all, level = "trace")]
                fn #method_idents() -> impl actix_web::dev::HttpServiceFactory + utoipa_actix_web::OpenApiFactory + 'static {
                    __routes::#method_idents
                }
            )*
        }
    };

    quote! {
        #routes_mod
        #handler_impl
    }
}

pub(super) fn process_regular_implementation(
    result_type: Ident,
    mut input_mod: ItemMod,
) -> TokenStream {
    let vis = &input_mod.vis;
    let mod_ident = &input_mod.ident;

    let items = if let Some((_, items)) = input_mod.content.take() {
        items
    } else {
        vec![]
    };

    let item_structs: Vec<_> = items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(s) => Some(s),
            _ => None,
        })
        .collect();

    let item_struct = match item_structs.len() {
        1 => *item_structs.first().unwrap(),
        _ => {
            return syn::Error::new(
                input_mod.span(),
                "implementation mod must have exactly one struct!",
            )
            .to_compile_error();
        },
    };

    let impl_name = item_struct.ident.clone();

    let struct_fields: Option<Vec<Field>> =
        if let Fields::Named(fields) = &item_struct.fields {
            Some(fields.named.clone().into_iter().collect())
        } else {
            return syn::Error::new(
                item_struct.span(),
                "Struct must have named fields!",
            )
            .to_compile_error();
        };

    let fields = match struct_fields {
        Some(fields) => fields,
        None => {
            return syn::Error::new(
                mod_ident.span(),
                "Regular implementation must contain a struct with fields",
            )
            .to_compile_error();
        },
    };

    let trait_impl = items.clone().into_iter().find_map(|item| match item {
        Item::Impl(mut imp) => {
            imp.items = imp
                .items
                .clone()
                .into_iter()
                .map(|item| match item {
                    ImplItem::Fn(mut func) => {
                        if wrap_return_type(&mut func.sig, &result_type) {
                            let body = func.block.clone();
                            func.block = parse_quote! {
                                {
                                     let res = #body;
                                    #[allow(unreachable_code)]
                                    Ok(res)
                                }
                            };
                        }
                        let trait_name = imp
                            .trait_
                            .clone()
                            .expect("impl should implement a trait")
                            .1
                            .segments
                            .last()
                            .expect("I don't think we can get empty segments here")
                            .ident
                            .to_string();
                        let func_name = func.sig.ident.to_string();
                        let trace_name = format!("{trait_name}::{func_name}");
                        func.attrs.push(parse_quote! { #[macros::instrument(skip_all, #trace_name)] });
                        ImplItem::Fn(func)
                    },
                    _ => item,
                })
                .collect();
            imp.attrs.push(parse_quote! { #[macros::async_trait] });
            Some(quote! { #imp })
        },
        _ => None,
    });

    let trait_impl_content =
        match trait_impl {
            Some(content) => content,
            None => return syn::Error::new(
                mod_ident.span(),
                "Regular implementation must contain a trait implementation",
            )
            .to_compile_error(),
        };

    let field_names = fields.iter().map(|f| &f.ident);
    let new_fn = quote! {
        impl #impl_name {
            #[tracing::instrument(skip_all, level = "trace")]
            pub fn new(#(#fields),*) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    #(#field_names),*
                })
            }
        }
    };

    let struct_def = quote! {
        #vis struct #impl_name {
            #(#fields),*
        }
    };

    quote! {
        #struct_def
        #new_fn
        #trait_impl_content
    }
}
