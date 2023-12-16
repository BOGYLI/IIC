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

// TODO: merge with DBCheckable - done?
#[proc_macro_derive(DBQueryable, attributes(sqlx_ext))]
pub fn derive(item: TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = ast.ident;
    let name_lower = quote::format_ident!("{}", name.to_string().to_lowercase());

    let names = split_by_uppercase(&name.to_string());
    let lowers = names.iter().map(|n| n.to_lowercase());
    let name_ids = lowers.map(|l| quote::format_ident!("{}id", l));

    let get_all = quote::quote!(
        fn get_all(conn: &mut PgConnection) -> Result<Vec<models::#name>, diesel::result::Error> {
            schema::#name_lower::dsl::#name_lower.load::<models::#name>(conn)
        }
    );
    let get = if names.len() > 1 {
        let name_ids = name_ids.clone();
        quote::quote!(
            fn get(&self, conn: &mut PgConnection) -> Result<models::#name, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                match #name_lower
                #(
                    .filter(#name_ids.eq(self.#name_ids))
                )*
                .load::<models::#name>(conn) {
                    Ok(data) => Ok(data[0].clone()),
                    Err(e) => Err(e)
                }
            }
        )
    } else {
        quote::quote!(
            fn get(&self, conn: &mut PgConnection) -> Result<models::#name, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                match #name_lower
                    .filter(id.eq(self.id))
                    .load::<models::#name>(conn) {
                        Ok(data) => Ok(data[0].clone()),
                        Err(e) => Err(e)
                    }
            }
        )
    };
    let delete = if names.len() > 1 {
        let name_ids = name_ids.clone();
        quote::quote!(
            fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                diesel::delete(#name_lower
                #(
                    .filter(#name_ids.eq(self.#name_ids))
                )*
                ).execute(conn)
            }
        )
    } else {
        quote::quote!(
            fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                diesel::delete(#name_lower.filter(id.eq(self.id))).execute(conn)
            }
        )
    };
    let update = if names.len() > 1 {
        let name_ids = name_ids.clone();
        quote::quote!(
            fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                diesel::update(#name_lower
                    #(
                        .filter(#name_ids.eq(self.#name_ids))
                    )*
                ).set(self).execute(conn)
            }
        )
    } else {
        quote::quote!(
            fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
                use schema::#name_lower::dsl::*;
                diesel::update(#name_lower
                    .filter(id.eq(self.id))
                ).set(self).execute(conn)
            }
        )
    };
    /*let update = quote::quote!(
        fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
            diesel::update(schema::#name_lower::table).set(self).execute(conn)
        }
    );*/

    quote::quote!{
        impl DBQueryable<models::#name, schema::#name_lower::SqlType> for models::#name {
            #get_all
            #get
            #delete
            #update
        }
    }.into()
}