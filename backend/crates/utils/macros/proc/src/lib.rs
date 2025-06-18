#![allow(clippy::unwrap_used)]
#![allow(clippy::arithmetic_side_effects)]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{ToTokens, format_ident, quote};
use syn::{
    DeriveInput, Item, ItemTrait, Meta, TraitItem, Type, parse_macro_input,
    punctuated::Punctuated, spanned::Spanned,
};

mod common;
mod entity;
mod handler;
mod implementation;
mod repository;

use entity::{EntityType, parse_entity};
use repository::{
    generate_crud_relation_repository, generate_crud_repository,
    generate_urd_relation_repository, generate_urd_repository,
};

use crate::{
    common::{
        parse_meta_name_value, wrap_return_type, wrap_trait_funcs_return_type,
    },
    handler::{process_routes_method, process_service_method},
    implementation::{
        parse_implementation_args, process_handler_implementation,
        process_regular_implementation,
    },
};

#[proc_macro_attribute]
pub fn entity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_entity(item, false)
}

#[proc_macro_attribute]
pub fn relation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_entity(item, true)
}

#[proc_macro_attribute]
pub fn repository(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let args_parser = Punctuated::<Meta, syn::Token![,]>::parse_terminated;
    let args = parse_macro_input!(attr with args_parser);

    let mut entity = None;
    let mut entity_id = None;
    let mut create_entity = None;
    let mut update_entity = None;
    let mut upsert_entity = None;
    let mut relation_in = None;
    let mut relation_out = None;
    let mut error_type = None;

    for arg in args {
        match arg {
            Meta::NameValue(nv) if nv.path.is_ident("entity") => {
                entity = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("entity_id") => {
                entity_id = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("create") => {
                create_entity = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("update") => {
                update_entity = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("upsert") => {
                upsert_entity = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("r#in") => {
                relation_in = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("out") => {
                relation_out = parse_meta_name_value(nv);
            },
            Meta::NameValue(nv) if nv.path.is_ident("error") => {
                error_type = parse_meta_name_value(nv)
            },
            _ => {},
        }
    }

    let is_relation = (relation_in.is_some() || relation_out.is_some())
        && entity_id.is_none();

    let entity_type = match (
        create_entity.is_some() || update_entity.is_some(),
        upsert_entity.is_some(),
    ) {
        (true, false) => EntityType::CRUD,
        (false, true) => EntityType::URD,
        (true, true) => return syn::Error::new(
            input.ident.span(),
            "Entity struct can't have upsert fields with create or update ones",
        )
        .to_compile_error()
        .into(),
        (false, false) => EntityType::None,
    };

    let entity = entity.expect("Entity must be specified");
    let error_type = error_type.expect("Error type must be specified");
    let trait_name = input.ident;

    let result_type = format_ident!("{trait_name}Result");

    let generated_methods = if is_relation {
        let in_type = relation_in.expect("Relation must specify 'r#in' type");
        let out_type = relation_out.expect("Relation must specify 'out' type");

        if entity_type == EntityType::CRUD {
            let create_entity = create_entity
                .clone()
                .expect("Create entity must be specified");
            let update_entity = update_entity
                .clone()
                .expect("Update entity must be specified");

            generate_crud_relation_repository(
                &in_type,
                &out_type,
                &entity,
                &create_entity,
                &update_entity,
                &result_type,
            )
        } else {
            let upsert_entity = upsert_entity
                .clone()
                .expect("Upsert entity must be specified");

            generate_urd_relation_repository(
                &in_type,
                &out_type,
                &entity,
                &upsert_entity,
                &result_type,
            )
        }
    } else {
        let entity_id = entity_id.clone().expect("EntityId must be specified");

        if entity_type == EntityType::CRUD {
            let create_entity = create_entity
                .clone()
                .expect("Create entity must be specified");
            let update_entity = update_entity
                .clone()
                .expect("Update entity must be specified");

            generate_crud_repository(
                &entity,
                &entity_id,
                &create_entity,
                &update_entity,
                &result_type,
            )
        } else {
            let upsert_entity = upsert_entity
                .clone()
                .expect("Upsert entity must be specified");

            generate_urd_repository(
                &entity,
                &entity_id,
                &upsert_entity,
                &result_type,
            )
        }
    };

    let original_contents =
        wrap_trait_funcs_return_type(input.items, &result_type);

    let dependency_type = format_ident!("{trait_name}Dependency");
    let error_type = syn::parse_str::<Type>(&error_type).unwrap();

    quote! {
        type #result_type<T> = Result<T, #error_type>;

        #[macros::async_trait]
        pub trait #trait_name {
            #generated_methods

            #(#original_contents)*
        }

        pub type #dependency_type = std::sync::Arc<dyn #trait_name + Send + Sync>;
    }.into()
}

#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(item as ItemTrait);

    let trait_vis = input_trait.vis.clone();
    let trait_ident = input_trait.ident.clone();
    let trait_generics = input_trait.generics.clone();
    let supertraits = input_trait.supertraits.clone();

    let args_parser = Punctuated::<Meta, syn::Token![,]>::parse_terminated;
    let args = parse_macro_input!(attr with args_parser);

    let mut error_type = None;

    for arg in args {
        match arg {
            Meta::NameValue(nv) if nv.path.is_ident("error") => {
                error_type = parse_meta_name_value(nv)
            },
            _ => {},
        }
    }

    let error_type = error_type.expect("Error type must be specified");

    let error_ident = syn::parse_str::<Type>(&error_type).unwrap();
    let result_ident =
        Ident::new(&format!("{trait_ident}Result"), trait_ident.span());
    let dependency_ident =
        Ident::new(&format!("{trait_ident}Dependency"), trait_ident.span());

    let items = wrap_trait_funcs_return_type(input_trait.items, &result_ident);

    let output = quote! {
        #trait_vis type #result_ident<T> = Result<T, #error_ident>;

        #[macros::async_trait]
        #trait_vis trait #trait_ident #trait_generics #supertraits {
            #(#items)*
        }

        #trait_vis type #dependency_ident = std::sync::Arc<dyn #trait_ident + std::marker::Send + std::marker::Sync>;
    };

    output.into()
}

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(item as ItemTrait);

    let ident = &input_trait.ident;
    let handler_ident = format_ident!("{ident}");
    let vis = &input_trait.vis;
    let generics = &input_trait.generics;
    let supertraits = &input_trait.supertraits;

    let args_parser = Punctuated::<Meta, syn::Token![,]>::parse_terminated;
    let args = parse_macro_input!(attr with args_parser);

    let mut error_type = None;

    for arg in args {
        match arg {
            Meta::NameValue(nv) if nv.path.is_ident("error") => {
                error_type = parse_meta_name_value(nv)
            },
            _ => {},
        }
    }

    let error_type = error_type.expect("Error type must be specified");

    let error_ident = syn::parse_str::<Type>(&error_type).unwrap();
    let result_ident = format_ident!("{ident}Result");

    let mut routes_method = None;
    let mut service_methods = Vec::new();

    for item in &input_trait.items {
        match item {
            TraitItem::Fn(func) => match func.sig.ident.to_string().as_str() {
                "routes" => {
                    routes_method = Some(process_routes_method(func));
                },
                _ => {
                    service_methods.push(process_service_method(func));
                },
            },
            _ => {
                return syn::Error::new(
                    item.span(),
                    "Only method items are allowed in handler trait",
                )
                .to_compile_error()
                .into();
            },
        }
    }

    let routes_method = routes_method.ok_or_else(|| {
        syn::Error::new(
            ident.span(),
            "Missing required 'routes' method in handler trait",
        )
    });

    let routes_method = match routes_method {
        Ok(method) => method,
        Err(e) => return e.to_compile_error().into(),
    };

    let helper_items = input_trait.items.into_iter().filter_map(|item| {
        match item {
            TraitItem::Fn(mut func) => {
                match func.sig.ident.to_string().as_str() {
                    "routes" => None,
                    _ => {
                        wrap_return_type(&mut func.sig, &result_ident);
                        Some(TraitItem::Fn(func))
                    },
                }
            },
            _ => Some(item),
        }
        .map(|item| item.to_token_stream())
    });

    let output = quote! {
        type #result_ident<T> = ::std::result::Result<T, #error_ident>;

        #vis trait #handler_ident #generics #supertraits {
            #(#service_methods)*

            #routes_method
        }

        #[macros::async_trait]
        trait __helper #generics #supertraits {
            #(#helper_items)*
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn implementation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args_parser = Punctuated::<Meta, syn::Token![,]>::parse_terminated;
    let args = parse_macro_input!(attr with args_parser);
    let (trait_name, impl_name, result_name) = parse_implementation_args(args);

    let input = parse_macro_input!(item as Item);

    match input {
        Item::Mod(module) => {
            let is_handler = match module.ident.to_string().as_str() {
                "repository" => false,
                "service" => false,
                "handler" => true,
                _ => return syn::Error::new(
                    module.ident.span(),
                    "module name should be `repository`, `service` or `handler`"
                ).to_compile_error().into(),
            };

            if is_handler {
                let trait_name = trait_name.expect("Missing trait name");
                let impl_name = impl_name.expect("Missing impl name");

                process_handler_implementation(
                    trait_name,
                    impl_name,
                    result_name,
                    module,
                )
            } else {
                process_regular_implementation(result_name, module)
            }
        },
        _ => syn::Error::new(
            input.span(),
            "Implementation must be attached to a module containing methods",
        )
        .to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(Secret)]
pub fn secret(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let debug_str = format!("{}(secret)", name);
    let display_str = format!("{name}");
    quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #debug_str.fmt(f)
            }
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #display_str.fmt(f)
            }
        }
    }
    .into()
}
