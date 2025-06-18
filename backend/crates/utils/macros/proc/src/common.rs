use proc_macro2::Ident;
use syn::{MetaNameValue, ReturnType, Signature, TraitItem, Type, parse_quote};

pub(super) fn parse_meta_name_value(nv: MetaNameValue) -> Option<String> {
    if let syn::Expr::Path(expr_path) = nv.value {
        return Some(expr_path.path.get_ident()?.to_string());
    }
    None
}

pub(super) fn wrap_trait_funcs_return_type(
    items: Vec<TraitItem>,
    result_type: &Ident,
) -> Vec<TraitItem> {
    items
        .into_iter()
        .map(|item| match item {
            TraitItem::Fn(mut func) => {
                wrap_return_type(&mut func.sig, result_type);
                TraitItem::Fn(func)
            },
            _ => item,
        })
        .collect()
}

pub(super) fn wrap_return_type(
    sig: &mut Signature,
    result_type: &Ident,
) -> bool {
    if let ReturnType::Type(_, ref mut ty) = sig.output {
        let new_ty: Type = parse_quote! { #result_type<#ty> };
        *ty = Box::new(new_ty);
        true
    } else {
        false
    }
}
