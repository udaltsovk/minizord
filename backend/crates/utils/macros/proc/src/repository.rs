use heck::ToSnakeCase;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub(super) fn generate_crud_repository(
    entity: &str,
    entity_id: &str,
    create_entity: &str,
    update_entity: &str,
    result_type: &Ident,
) -> proc_macro2::TokenStream {
    let entity = format_ident!("{entity}");
    let entity_id = format_ident!("{entity_id}");

    let create_entity = format_ident!("{create_entity}");
    let update_entity = format_ident!("{update_entity}");

    quote! {
        async fn save(
            &self,
            object: #create_entity
        ) -> #result_type<#entity>;

        async fn find_by_id(
            &self,
            id: #entity_id
        ) -> #result_type<Option<#entity>>;

        async fn exists_by_id(
            &self,
            id: #entity_id
        ) -> #result_type<bool>;

        async fn update_by_id(
            &self,
            id: #entity_id,
            update: #update_entity
        ) -> #result_type<Option<#entity>>;

        async fn delete_by_id(
            &self,
            id: #entity_id
        ) -> #result_type<Option<#entity>>;
    }
}

pub(super) fn generate_urd_repository(
    entity: &str,
    entity_id: &str,
    upsert_entity: &str,
    result_type: &Ident,
) -> proc_macro2::TokenStream {
    let entity = format_ident!("{entity}");
    let entity_id = format_ident!("{entity_id}");

    let upsert_entity = format_ident!("{upsert_entity}");

    quote! {
        async fn upsert_by_id(
            &self,
            id: #entity_id,
            object: #upsert_entity
        ) -> #result_type<#entity>;

        async fn find_by_id(
            &self,
            id: #entity_id
        ) -> #result_type<Option<#entity>>;

        async fn exists_by_id(
            &self,
            id: #entity_id
        ) -> #result_type<bool>;

        async fn delete_by_id(
            &self,
            id: #entity_id
        ) -> #result_type<Option<#entity>>;
    }
}

pub(super) fn generate_crud_relation_repository(
    in_type: &str,
    out_type: &str,
    entity: &str,
    create_entity: &str,
    update_entity: &str,
    result_type: &Ident,
) -> proc_macro2::TokenStream {
    let in_type = syn::parse_str::<Type>(in_type).unwrap();
    let out_type = syn::parse_str::<Type>(out_type).unwrap();

    let entity_snake = format_ident!("{}", entity.to_snake_case());
    let entity = format_ident!("{entity}");
    let create_entity = format_ident!("{create_entity}");
    let update_entity = format_ident!("{update_entity}");

    quote! {
        fn get_id_string(&self, in_id: &#in_type, out_id: &#out_type) -> String {
            format!("{in_id}_{out_id}")
        }

        fn get_id(&self, in_id: &#in_type, out_id: &#out_type) -> surrealdb::RecordId {
            surrealdb::RecordId::from_table_key(stringify!(#entity_snake), self.get_id_string(in_id, out_id))
        }

        async fn save(
            &self,
            object: #create_entity
        ) -> #result_type<#entity>;

        async fn find_all_by_in(
            &self,
            r#in: #in_type,
            limit: u16,
            offset: u64
        ) -> #result_type<Vec<#entity>>;

        async fn exists_by_in(
            &self,
            r#in: #in_type
        ) -> #result_type<bool>;

        async fn find_all_by_out(
            &self,
            out: #out_type,
            limit: u16,
            offset: u64
        ) -> #result_type<Vec<#entity>>;

        async fn exists_by_out(
            &self,
            out: #out_type
        ) -> #result_type<bool>;

        async fn find_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type
        ) -> #result_type<Option<#entity>>;

        async fn exists_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type
        ) -> #result_type<bool>;

        async fn update_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type,
            update: #update_entity
        ) -> #result_type<Option<#entity>>;

        async fn delete_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type
        ) -> #result_type<Option<#entity>>;
    }
}

pub(super) fn generate_urd_relation_repository(
    in_type: &str,
    out_type: &str,
    entity: &str,
    upsert_entity: &str,
    result_type: &Ident,
) -> proc_macro2::TokenStream {
    let in_type = syn::parse_str::<Type>(in_type).unwrap();
    let out_type = syn::parse_str::<Type>(out_type).unwrap();

    let entity_snake = format_ident!("{}", entity.to_snake_case());
    let entity = format_ident!("{entity}");

    let upsert_entity = format_ident!("{upsert_entity}");

    quote! {
        fn get_id_string(&self, in_id: &#in_type, out_id: &#out_type) -> String {
            format!("{in_id}_{out_id}")
        }

        fn get_id(&self, in_id: &#in_type, out_id: &#out_type) -> surrealdb::RecordId {
            surrealdb::RecordId::from_table_key(stringify!(#entity_snake), self.get_id_string(in_id, out_id))
        }

        async fn upsert_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type,
            object: #upsert_entity
        ) -> #result_type<#entity>;

        async fn find_all_by_in(
            &self,
            r#in: #in_type,
            limit: u16,
            offset: u64
        ) -> #result_type<Vec<#entity>>;

        async fn exists_by_in(
            &self,
            r#in: #in_type
        ) -> #result_type<bool>;

        async fn find_all_by_out(
            &self,
            out: #out_type,
            limit: u16,
            offset: u64
        ) -> #result_type<Vec<#entity>>;

        async fn exists_by_out(
            &self,
            out: #out_type
        ) -> #result_type<bool>;

        async fn find_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type
        ) -> #result_type<Option<#entity>>;

        async fn exists_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type
        ) -> #result_type<bool>;

        async fn delete_by_in_and_out(
            &self,
            r#in: #in_type,
            out: #out_type
        ) -> #result_type<Option<#entity>>;
    }
}
