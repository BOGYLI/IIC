use proc_macro::TokenStream;
use quote::format_ident;

struct ConnectionTable(syn::Type, syn::Type);

impl syn::parse::Parse for ConnectionTable {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::parse::Error> {
        let first_type: syn::Type = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let second_type: syn::Type = input.parse()?;
        Ok(ConnectionTable(first_type, second_type))
    }
}

#[proc_macro]
pub fn connection(input: TokenStream) -> TokenStream {
    //let ast: syn::Type = syn::parse(input).unwrap();
    let ast = syn::parse_macro_input!(input as ConnectionTable);
    let names = (
        match &ast.0 {
            syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
            _ => todo!("!!!"),
        },
        match &ast.1 {
            syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
            _ => todo!("!!!"),
        }
    );
    let names_lower = (names.0.to_lowercase(), names.1.to_lowercase());
    //let idents_lower = (format_ident!("{}", names_lower.0), format_ident!("{}", names_lower.1));
    let first = ast.0;
    let second = ast.1;
    let get_connected_with_first = format_ident!("get_connected_with_{}", names_lower.0);
    let get_connected_with_second = format_ident!("get_connected_with_{}", names_lower.1);
    let firstid = format_ident!("{}id", names_lower.0);
    let secondid = format_ident!("{}id", names_lower.1);
    let connection = format_ident!("{}{}", names_lower.0, names_lower.1);
    let first_l = format_ident!("{}", names_lower.0);
    let second_l = format_ident!("{}", names_lower.1);

    let first_impl = quote::quote!(
        impl #first {
            pub fn #get_connected_with_second(id: i32, conn: &mut diesel::PgConnection) -> Result<Vec<#first>, diesel::result::Error> {
                use crate::db::schema::#second_l::dsl::{#second_l};
                use crate::db::schema::#first_l::dsl::{#first_l};
                use crate::db::schema::#connection::dsl::{#connection, #secondid};
                //let conn = &mut crate::db::establish_connection();
                #connection.inner_join(#first_l).inner_join(#second_l).filter(#secondid.eq(id)).select(#first::as_select()).load::<crate::db::models::#first>(conn)
            }
        }
    );

    let second_impl = quote::quote!(
        impl #second {
            pub fn #get_connected_with_first(id: i32, conn: &mut diesel::PgConnection) -> Result<Vec<#second>, diesel::result::Error> {
                use crate::db::schema::#second_l::dsl::{#second_l};
                use crate::db::schema::#first_l::dsl::{#first_l};
                use crate::db::schema::#connection::dsl::{#connection, #firstid};
                //let conn = &mut crate::db::establish_connection();
                #connection.inner_join(#second_l).inner_join(#first_l).filter(#firstid.eq(id)).select(#second::as_select()).load::<crate::db::models::#second>(conn)
            }
        }
    );

    quote::quote!(
        #first_impl

        #second_impl
    ).into()
}