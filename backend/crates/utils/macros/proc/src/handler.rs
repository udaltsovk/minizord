use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, TraitItemFn, parse_quote, spanned::Spanned};

pub(super) fn process_routes_method(method: &TraitItemFn) -> TokenStream {
    let attrs = &method.attrs;
    let sig = &method.sig;
    let block = &method.default;

    let params = &sig.inputs;

    if params.is_empty() {
        return syn::Error::new(
            params.span(),
            "Routes method must have at least one parameter: &mut ServiceConfig"
        )
            .to_compile_error();
    }

    let last_index = params.len() - 1;
    let capture_params =
        params.iter().take(last_index).cloned().collect::<Vec<_>>();
    let service_config_param = params.iter().next_back().unwrap();

    let service_config_pat = match service_config_param {
        FnArg::Typed(pat_type) => {
            if !is_service_config_type(&pat_type.ty) {
                return syn::Error::new(
                    pat_type.ty.span(),
                    "Last parameter must be &mut ServiceConfig",
                )
                .to_compile_error();
            }
            &pat_type.pat
        },
        _ => {
            return syn::Error::new(
                service_config_param.span(),
                "Last parameter must be a value parameter",
            )
            .to_compile_error();
        },
    };

    let mut new_sig = sig.clone();
    new_sig.inputs = capture_params.clone().into_iter().collect();

    let mut where_clause = new_sig
        .generics
        .where_clause
        .clone()
        .unwrap_or_else(|| parse_quote!(where));
    where_clause
        .predicates
        .push(parse_quote!(Self: Sized + Clone + 'static));

    new_sig.generics.where_clause = Some(where_clause);
    new_sig.output = parse_quote! {
        -> impl FnOnce(&mut ServiceConfig)
    };

    let closure = quote! {
        {
            move |__cfg: &mut ServiceConfig| {
                let #service_config_pat = __cfg;

                #block
            }
        }
    };

    quote! {
        #(#attrs)*
        #new_sig #closure
    }
}

fn is_service_config_type(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(type_ref) = ty {
        if type_ref.mutability.is_some() {
            if let syn::Type::Path(type_path) = &*type_ref.elem {
                if let Some(segment) = type_path.path.segments.last() {
                    return segment.ident == "ServiceConfig";
                }
            }
        }
    }
    false
}

pub(super) fn process_service_method(func: &TraitItemFn) -> TokenStream {
    let attrs = &func.attrs;
    let ident = &func.sig.ident;

    quote! {
        #(#attrs)*
        fn #ident() -> impl ::actix_web::dev::HttpServiceFactory + ::utoipa_actix_web::OpenApiFactory + 'static;
    }
}
