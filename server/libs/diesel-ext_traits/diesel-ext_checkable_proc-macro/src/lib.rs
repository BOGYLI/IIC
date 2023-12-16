use proc_macro::TokenStream;
use syn::DeriveInput;

fn split_by_uppercase(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;
    for (i, c) in input.char_indices() {
        if c.is_uppercase() {
            if i > start {
                result.push(input[start..i].to_string());
            }
            start = i;
        }
    }
    if start < input.len() {
        result.push(input[start..].to_string());
    }
    result
}

// TODO: beliebige länge des natürlichen Schlüssels unterstützen (Loop)
#[proc_macro_derive(DBCheckable, attributes(sqlx_ext))]
pub fn derive(item: TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = ast.ident;
    let names = split_by_uppercase(&name.to_string());
    let name_lower = quote::format_ident!("{}", name.to_string().to_lowercase());
    let lower1 = quote::format_ident!("{}", names[0].to_lowercase());
    let lower2 = quote::format_ident!("{}", names[1].to_lowercase());
    let one_id = quote::format_ident!("{}id", &lower1);
    let two_id = quote::format_ident!("{}id", &lower2);

    quote::quote!{
        impl DBCheckable<models::#name, schema::#name_lower::SqlType> for models::#name {
            fn check(&self, conn: &mut PgConnection) -> Result<models::#name, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                match #name_lower
                    .filter(#one_id.eq(self.#one_id))
                    .filter(#two_id.eq(self.#two_id))
                .load::<models::#name>(conn) {
                    Ok(data) => Ok(data[0].clone()),
                    Err(e) => Err(e)
                }
            }
        }
    }.into()
}