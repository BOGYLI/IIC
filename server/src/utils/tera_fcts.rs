use std::collections::BTreeMap;
use tera::{self, Value, from_value, to_value};
use tera::Function;
use std::collections::HashMap;
use tera::Number;

pub fn media_url(_urls: BTreeMap<String, String>) -> impl Function {
    Box::new(move |args: &HashMap<std::string::String, Value>| -> Result<Value, tera::Error> {
        match args.get("id") {
            Some(val) => match from_value::<Number>(val.clone()) {
                Ok(v) =>  {
                    let num = from_value::<Number>(tera::Value::Number(v)).unwrap();
                    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
                    let valu = to_value(wp_lib::Media::from(&uri, num.to_string()).unwrap().guid.rendered).unwrap();
                    Ok(valu)},
                Err(e) => Err(e.into()),
            },
            None => Err("oops".into()),
        }
    })
}

pub fn format_date(_urls: BTreeMap<String, String>) -> impl Function {
    Box::new(move |args: &HashMap<std::string::String, Value>| -> Result<Value, tera::Error> {
        match args.get("date") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) =>  {
                    let date = v.split('T').collect::<Vec<&str>>();
                    Ok(to_value(date[0]).unwrap())
                },
                Err(e) => Err(e.into()),
            },
            None => Err("oops".into()),
        }
    })
}