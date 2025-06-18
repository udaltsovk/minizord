use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, Type, parse_macro_input,
};

pub(super) fn parse_entity(
    item: TokenStream,
    is_relation: bool,
) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    if is_relation {
        relation_entity(input)
    } else {
        normal_entity(input)
    }
}

fn normal_entity(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let table_name = struct_name.to_string().to_snake_case();
    let id_name = format_ident!("{}Id", struct_name);

    let fields = if let Data::Struct(s) = input.data.clone() {
        s.fields
    } else {
        return syn::Error::new(
            input.ident.span(),
            "Entity macro only supports structs",
        )
        .to_compile_error()
        .into();
    };

    let id_type = match find_special_field(&fields, "id") {
        Some(ty) => ty,
        None => {
            return syn::Error::new(
                struct_name.span(),
                "Struct must have an 'id' field",
            )
            .to_compile_error()
            .into();
        },
    };

    let (
        main_fields,
        create_fields,
        update_fields,
        upsert_fields,
        create_name,
        update_name,
        upsert_name,
        entity_type,
    ) = match process_struct(&input, struct_name, &fields, &["r#in", "out"]) {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };

    let create_struct = if entity_type == EntityType::CRUD
        || entity_type == EntityType::None
    {
        Some(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
            pub struct #create_name {
                #(#create_fields)*
            }
        })
    } else {
        None
    };

    let update_struct = if entity_type == EntityType::CRUD
        || entity_type == EntityType::None
    {
        Some(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
            pub struct #update_name {
                #(#update_fields)*
            }
        })
    } else {
        None
    };

    let upsert_struct = if entity_type == EntityType::URD {
        Some(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
            pub struct #upsert_name {
                #(#upsert_fields)*
            }
        })
    } else {
        None
    };

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        #[serde(transparent)]
        pub struct #id_name(
            std::sync::Arc<surrealdb::RecordId>,
        );

        impl EntityId for #id_name {
            const TABLE: &'static str = #table_name;

            #[tracing::instrument(skip_all, level = "trace")]
            fn record_id(&self) -> surrealdb::RecordId {
                self.0.as_ref().clone()
            }
        }

        impl From<#id_type> for #id_name {
            #[tracing::instrument(skip_all, level = "trace")]
            fn from(id: #id_type) -> Self {
                Self(std::sync::Arc::new(
                    surrealdb::RecordId::from_table_key(Self::TABLE, id.to_string())
                ))
            }
        }

        impl std::fmt::Display for #id_name {
            #[tracing::instrument(skip_all, level = "trace")]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.key())
            }
        }

        impl From<#id_name> for surrealdb::RecordId {
            #[tracing::instrument(skip_all, level = "trace")]
            fn from(id: #id_name) -> Self {
                id.record_id()
            }
        }

        impl From<#id_name> for #id_type {
            #[tracing::instrument(skip_all, level = "trace")]
            fn from(id: #id_name) -> Self {
                use std::str::FromStr;
                let name = stringify!(#id_name);
                Self::from_str(&id.to_string()).expect("Got invalid {name}")
            }
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        pub struct #struct_name {
            pub id: #id_name,
            #(#main_fields)*
        }

        #create_struct
        #update_struct
        #upsert_struct
    };

    expanded.into()
}

fn relation_entity(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let table_name = struct_name.to_string().to_snake_case();
    let id_name = format_ident!("{}Id", struct_name);

    let fields = if let Data::Struct(s) = input.data.clone() {
        s.fields
    } else {
        return syn::Error::new(
            input.ident.span(),
            "Relation entity must be a struct",
        )
        .to_compile_error()
        .into();
    };

    let in_type = match find_special_field(&fields, "r#in") {
        Some(ty) => ty,
        None => {
            return syn::Error::new(
                struct_name.span(),
                "Relation must have an 'in' field",
            )
            .to_compile_error()
            .into();
        },
    };

    let out_type = match find_special_field(&fields, "out") {
        Some(ty) => ty,
        None => {
            return syn::Error::new(
                struct_name.span(),
                "Relation must have an 'out' field",
            )
            .to_compile_error()
            .into();
        },
    };

    let (
        main_fields,
        create_fields,
        update_fields,
        upsert_fields,
        create_name,
        update_name,
        upsert_name,
        entity_type,
    ) = match process_struct(&input, struct_name, &fields, &["r#in", "out"]) {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };

    let create_struct = if entity_type == EntityType::CRUD
        || entity_type == EntityType::None
    {
        Some(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
            pub struct #create_name {
                pub r#in: #in_type,
                pub out: #out_type,
                #(#create_fields)*
            }

            impl #create_name {
                #[tracing::instrument(skip_all, level = "trace")]
                pub fn get_id_string(&self) -> String {
                    format!("{}_{}", self.r#in, self.out)
                }

                #[tracing::instrument(skip_all, level = "trace")]
                pub fn get_id(&self) -> #id_name {
                    surrealdb::RecordId::from_table_key(<#id_name as EntityId>::TABLE, self.get_id_string()).into()
                }
            }
        })
    } else {
        None
    };

    let update_struct = if entity_type == EntityType::CRUD
        || entity_type == EntityType::None
    {
        Some(quote! {
        # [derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct # update_name {
        # ( # update_fields) *
        }
        })
    } else {
        None
    };

    let upsert_struct = if entity_type == EntityType::URD {
        Some(quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
            pub struct #upsert_name {
                pub r#in: #in_type,
                pub out: #out_type,
                #(#upsert_fields)*
            }

            impl #upsert_name {
                #[tracing::instrument(skip_all, level = "trace")]
                pub fn get_id_string(&self) -> String {
                    format!("{}_{}", self.r#in, self.out)
                }

                #[tracing::instrument(skip_all, level = "trace")]
                pub fn get_id(&self) -> #id_name {
                    surrealdb::RecordId::from_table_key(<#id_name as EntityId>::TABLE, self.get_id_string()).into()
                }
            }
        })
    } else {
        None
    };

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        #[serde(transparent)]
        pub struct #id_name(
             std::sync::Arc<surrealdb::RecordId>,
        );

        impl EntityId for #id_name {
            const TABLE: &'static str = #table_name;

            #[tracing::instrument(skip_all, level = "trace")]
            fn record_id(&self) -> surrealdb::RecordId {
                self.0.as_ref().clone()
            }
        }

        impl From<surrealdb::RecordId> for #id_name {
            #[tracing::instrument(skip_all, level = "trace")]
            fn from(id: surrealdb::RecordId) -> Self {
                Self(std::sync::Arc::new(id))
            }
        }

        impl std::fmt::Display for #id_name {
            #[tracing::instrument(skip_all, level = "trace")]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.key())
            }
        }

        impl From<#id_name> for surrealdb::RecordId {
            #[tracing::instrument(skip_all, level = "trace")]
            fn from(id: #id_name) -> Self {
                id.record_id()
            }
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        pub struct #struct_name {
            pub id: #id_name,
            pub r#in: #in_type,
            pub out: #out_type,
            #(#main_fields)*
        }

        #create_struct
        #update_struct
        #upsert_struct
    };

    expanded.into()
}

type ProcessedStruct = (
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Ident,
    Ident,
    Ident,
    EntityType,
);

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Debug)]
pub(crate) enum EntityType {
    None,
    CRUD,
    URD,
}

fn process_struct(
    input: &DeriveInput,
    name: &Ident,
    fields: &Fields,
    special_fields: &[&str],
) -> Result<ProcessedStruct, syn::Error> {
    let mut main_fields = vec![];
    let mut create_fields = vec![];
    let mut update_fields = vec![];
    let mut upsert_fields = vec![];

    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            if special_fields.contains(&&*ident.to_string()) {
                continue;
            }

            let field_attrs: Vec<&Attribute> = field
                .attrs
                .iter()
                .filter(|attr| {
                    !attr.path().is_ident("field")
                        && !attr.path().is_ident("create")
                        && !attr.path().is_ident("upsert")
                        && !attr.path().is_ident("update")
                })
                .collect();
            let field_name = ident.clone();
            let field_type = &field.ty;

            let mut in_main = false;
            let mut in_create = false;
            let mut in_update = false;
            let mut in_upsert = false;

            for attr in &field.attrs {
                if attr.path().is_ident("field") {
                    in_main = true;
                }
                if attr.path().is_ident("create") {
                    in_create = true;
                }
                if attr.path().is_ident("update") {
                    in_update = true;
                }
                if attr.path().is_ident("upsert") {
                    in_upsert = true;
                }
            }

            if in_main {
                main_fields.push(quote! {
                    #(#field_attrs)*
                    pub #field_name: #field_type,
                });
            }

            if in_create {
                create_fields.push(quote! {
                    #(#field_attrs)*
                    pub #field_name: #field_type,
                });
            }

            if in_update {
                update_fields.push(quote! {
                    #(#field_attrs)*
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #field_name: Option<#field_type>,
                });
            }

            if in_upsert {
                upsert_fields.push(quote! {
                    #(#field_attrs)*
                    pub #field_name: #field_type,
                });
            }
        }
    }

    let create_name = format_ident!("Create{}", name);
    let update_name = format_ident!("{}Update", name);
    let upsert_name = format_ident!("Upsert{}", name);

    let entity_type = match (
        create_fields.is_empty() && update_fields.is_empty(),
        upsert_fields.is_empty(),
    ) {
        (false, true) => EntityType::CRUD,
        (true, false) => EntityType::URD,
        (false, false) => Err(syn::Error::new(
            input.ident.span(),
            "Entity struct can't have upsert fields with create or update ones",
        ))?,
        (true, true) => EntityType::None,
    };

    Ok((
        main_fields,
        create_fields,
        update_fields,
        upsert_fields,
        create_name,
        update_name,
        upsert_name,
        entity_type,
    ))
}

fn find_special_field<'a>(fields: &'a Fields, name: &str) -> Option<&'a Type> {
    fields.iter().find_map(|f| {
        f.ident
            .as_ref()
            .and_then(|ident| if ident == name { Some(&f.ty) } else { None })
    })
}
