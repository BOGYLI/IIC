use proc_macro::TokenStream;
use quote::format_ident;

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

#[proc_macro]
pub fn delete(input: TokenStream) -> TokenStream {
    let ast: syn::Type = syn::parse(input).unwrap();
    let d = match &ast {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let name_lower = d.to_lowercase();
    let ident_lower = format_ident!("{}", name_lower);

    let names = split_by_uppercase(&d);
    let lowers = names.iter().map(|n| n.to_lowercase());
    let name_ids = lowers.map(|l| quote::format_ident!("{}id", l));
    let name_ids2 = name_ids.clone();
    let name_ids3 = name_ids.clone();

    let id_struct_ident = quote::format_ident!("{}IDs", &d);
    let id_struct = quote::quote!(
        #[derive(Serialize, Deserialize)]
        struct #id_struct_ident {
            #(
                pub #name_ids: i32,
            )*
        }
    );

    let path = if names.len() > 1 {
        format!("/{}", &name_lower)
    } else {
        format!("/{}/<id>", &name_lower)
    };
    let func = quote::format_ident!("delete_{}", &name_lower);
    //let imp = quote::format_ident!("schema::{}::dsl", &name_lower);

    let tmp = if names.len() > 1 {
        quote::quote!(
            #id_struct

            #[actix_web::post(#path)]
            pub async fn #func(ids: actix_web::web::Json<#id_struct_ident>) -> impl actix_web::Responder {
                let d = ids.into_inner();
                use diesel::prelude::*;
                use crate::db::schema::#ident_lower::dsl::*;
                //use schema::#name_lower;
                match diesel::delete(#ident_lower
                #(
                    .filter(#name_ids2.eq(d.#name_ids3))
                )*
                ).execute(&mut crate::db::establish_connection()) {
                    Ok(data) => Ok(actix_web::web::Json(data)),
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
                }
            }
        )
    } else {
        quote::quote!(
            #[actix_web::post(#path)]
            pub async fn #func(path: actix_web::web::Path<i32>) -> impl actix_web::Responder {
                let id_ = path.into_inner();
                use diesel::prelude::*;
                use crate::db::schema::#ident_lower::dsl::*;
                match diesel::delete(#ident_lower
                    .filter(id.eq(id_))
                ).execute(&mut crate::db::establish_connection()) {
                    Ok(data) => Ok(actix_web::web::Json(data)),
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
                }
            }
        )
    };
    quote::quote!(#tmp).into()
}

#[proc_macro]
pub fn update(input: TokenStream) -> TokenStream {
    let ast: syn::Type = syn::parse(input).unwrap();
    let d = match &ast {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    }.to_lowercase();
    let path = format!("/{}", &d);
    let func = format_ident!("update_{}", &d);
    quote::quote!(
        #[actix_web::post(#path)]
        pub async fn #func(data: actix_web::web::Json<#ast>) -> impl actix_web::Responder {
            match data.into_inner().update(&mut crate::db::establish_connection()) {
                Ok(data) => Ok(actix_web::web::Json(data)),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    ).into()
}

#[proc_macro]
pub fn get_all(input: TokenStream) -> TokenStream {
    let ast: syn::Type = syn::parse(input).unwrap();
    let d = match &ast {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    }.to_lowercase();
    let path = format!("/{}", &d);
    let func = format_ident!("get_all_{}", &d);
    quote::quote!(
        #[actix_web::get(#path)]
        pub async fn #func() -> impl actix_web::Responder {
            match #ast::get_all(&mut crate::db::establish_connection()) {
                Ok(data) => Ok(actix_web::web::Json(data)),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    ).into()
}

#[proc_macro]
pub fn get(input: TokenStream) -> TokenStream {
    let ast: syn::Type = syn::parse(input).unwrap();
    let d = match &ast {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let name_lower = d.to_lowercase();
    let ident_lower = format_ident!("{}", name_lower);

    let names = split_by_uppercase(&d);
    let lowers = names.iter().map(|n| n.to_lowercase());
    let name_ids = lowers.map(|l| quote::format_ident!("{}id", l));
    let name_ids2 = name_ids.clone();
    let name_ids3 = name_ids.clone();

    let id_struct_ident = quote::format_ident!("{}IDs", &d);
    let id_struct = quote::quote!(
        #[derive(Serialize, Deserialize)]
        struct #id_struct_ident {
            #(
                pub #name_ids: i32,
            )*
        }
    );

    let path = if names.len() > 1 {
        format!("/{}", &name_lower)
    } else {
        format!("/{}/<id>", &name_lower)
    };
    let func = quote::format_ident!("get_{}", &name_lower);
    //let imp = quote::format_ident!("schema::{}::dsl", &name_lower);

    let tmp = if names.len() > 1 {
        quote::quote!(
            #id_struct

            #[actix_web::post(#path)]
            pub async fn #func(ids: actix_web::web::Json<#id_struct_ident>) -> impl actix_web::Responder {
                let d = ids.into_inner();
                use diesel::prelude::*;
                use crate::db::schema::#ident_lower::dsl::*;
                //use schema::#name_lower;
                match #ident_lower
                #(
                    .filter(#name_ids2.eq(d.#name_ids3))
                )*
                .load::<#ast>(&mut crate::db::establish_connection()) {
                    Ok(data) => Ok(actix_web::web::Json(data)),
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
                }
            }
        )
    } else {
        quote::quote!(
            #[actix_web::post(#path)]
            pub async fn #func(path: actix_web::web::Path<i32>) -> impl actix_web::Responder {
                let id_ = path.into_inner();
                use diesel::prelude::*;
                use crate::db::schema::#ident_lower::dsl::*;
                match #ident_lower
                    .filter(id.eq(id_))
                .load::<#ast>(&mut crate::db::establish_connection()) {
                    Ok(data) => Ok(actix_web::web::Json(data)),
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
                }
            }
        )
    };
    quote::quote!(#tmp).into()
}
/*
#[proc_macro]
pub fn get(input: TokenStream) -> TokenStream {
    let ast: syn::Type = syn::parse(input).unwrap();
    let d = match &ast {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    }.to_lowercase();
    let path = format!("/{}", &d);
    let func = format_ident!("get_{}", &d);
    quote::quote!(
        #[actix_web::get(#path)]
        pub async fn #func() -> impl actix_web::Responder {
            let id = path.into_inner();
            match #ast::new_by_id(id).get(&mut crate::db::establish_connection()) {
                Ok(data) => actix_web::web::Json(data).finish(),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    ).into()
}*/

#[proc_macro]
pub fn insert(input: TokenStream) -> TokenStream {
    let ast: syn::Type = syn::parse(input).unwrap();
    let name = match &ast {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let d = name.to_lowercase();
    let new_ast = format_ident!("New{}", &name);
    let path = format!("/{}", &d);
    let func = format_ident!("new_{}", &d);
    quote::quote!(
        #[actix_web::post(#path)]
        pub async fn #func(data: actix_web::web::Json<#new_ast>) -> impl actix_web::Responder {
            use diesel_ext_traits::DBInsertable;
            match data.into_inner().new(&mut crate::db::establish_connection()) {
                Ok(data) => Ok(actix_web::web::Json(data)),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    ).into()
}

/*


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
pub fn delete_connection(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as ConnectionTable);
    let one = match &ast.0 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower1 = one.to_lowercase();
    let two = match &ast.1 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower2 = two.to_lowercase();
    let path = format!("/{}{}", &lower1, &lower2);
    let func = format_ident!("delete_{}{}", &lower1, &lower2);
    let s = format_ident!("{}{}", &one, &two);
    quote::quote!(
        #[actix_web::post(#path)]
        pub async fn #func(data: actix_web::web::Json<#s>) -> impl actix_web::Responder {
            match data.into_inner().delete(&mut crate::db::establish_connection()) {
                Ok(data) => actix_web::web::Json(data).finish(),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    ).into()
}

#[proc_macro]
pub fn get_all_connection(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as ConnectionTable);
    let one = match &ast.0 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower1 = one.to_lowercase();
    let two = match &ast.1 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower2 = two.to_lowercase();
    let path = format!("/{}{}", &lower1, &lower2);
    let func = format_ident!("get_all_{}{}", &lower1, &lower2);
    let s = format_ident!("{}{}", &one, &two);
    quote::quote!(
        #[actix_web::post(#path)]
        pub async fn #func(data: actix_web::web::Json<#s>) -> impl actix_web::Responder {
            match #s::get_all(&mut crate::db::establish_connection()) {
                Ok(data) => actix_web::web::Json(data).finish(),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    ).into()
}

#[proc_macro]
pub fn get_connection(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as ConnectionTable);
    let one = match &ast.0 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower1 = one.to_lowercase();
    let two = match &ast.1 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower2 = two.to_lowercase();
    let path = format!("/{}{}", &lower1, &lower2);
    let func = format_ident!("get_{}{}", &lower1, &lower2);
    let s = format_ident!("{}{}", &one, &two);
    quote::quote!(
        #[actix_web::post(#path)]
        pub async fn #func(data: actix_web::web::Json<#s>) -> impl actix_web::Responder {
            match data.into_inner().exists(&mut crate::db::establish_connection()) {
                Ok(data) => actix_web::web::Json(data).finish(),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    ).into()
}

#[proc_macro]
pub fn new_connection(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as ConnectionTable);
    let one = match &ast.0 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower1 = one.to_lowercase();
    let two = match &ast.1 {
        syn::Type::Path(p) => format!("{}", p.path.segments.last().unwrap().ident),
        _ => todo!("!!!"),
    };
    let lower2 = two.to_lowercase();
    let path = format!("/{}{}", &lower1, &lower2);
    let func = format_ident!("new_{}{}", &lower1, &lower2);
    let s = format_ident!("{}{}", &one, &two);
    quote::quote!(
        #[actix_web::post(#path)]
        pub async fn #func(data: actix_web::web::Json<#s>) -> impl actix_web::Responder {
            match data.into_inner().new(&mut crate::db::establish_connection()) {
                Ok(data) => actix_web::web::Json(data).finish(),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish()
            }
        }
    ).into()
}

*/