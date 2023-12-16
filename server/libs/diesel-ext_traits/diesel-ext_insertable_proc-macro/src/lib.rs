use proc_macro::TokenStream;
use syn::DeriveInput;



#[proc_macro_derive(DBInsertable, attributes(sqlx_ext))]
pub fn derive(item: TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = ast.ident;
    let new_name = quote::format_ident!("New{}", name);
    let name_lower = quote::format_ident!("{}", name.to_string().to_lowercase());

    quote::quote!{
        impl DBInsertable<models::#name, schema::#name_lower::SqlType> for models_new::#new_name {
            fn new(self, conn: &mut diesel::PgConnection) -> Result<models::#name, diesel::result::Error> {
                diesel::insert_into(crate::db::schema::#name_lower::table).values(self).get_result(conn)
            }
        }
    }.into()
}


