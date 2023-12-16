use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Fields, Ident, Attribute, spanned::Spanned, ExprLit, DataStruct};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(restapi))]
struct RestApiAttributes {
    base_path: String,
}

/*#[proc_macro_attribute]
pub fn insert(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(item).unwrap();
    let mut attr: DeriveInput = syn::parse(attr).unwrap();
    
    //let attr: InsertAttributes = deluxe::extract_attributes(&mut attr).unwrap();
    if let syn::Meta::NameValue(v)  = &attr.attrs[0].meta {
        unimplemented!("{:?}", v.value)
    }
    
    quote::quote!(
        //#[get(format!("#[get({}/{})]",  $attr.base_path, $ast.ident))]
        fn insert() {}
    ).into()
}*/





#[proc_macro_derive(RestApi, attributes(restapi))]
pub fn derive(item: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(item).unwrap();
    let attrs: RestApiAttributes = deluxe::extract_attributes(&mut ast).unwrap();
    let name = ast.ident;

    quote::quote!(
        impl $ident {
            #[test]
            fn t() {}
        }
    ).into()
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {