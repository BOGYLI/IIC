use ureq;
use serde_json::{Result, Value};
use serde_derive::{Deserialize, Serialize};
use cache_lib as cache;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guid {
    pub rendered: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Title {
    pub rendered: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub wp_typography_post_enhancements_disabled: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub rendered: String,
    pub protected: bool
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Excerpt {
    pub rendered: String,
    pub protected: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub date: Option<String>,
    pub date_gmt: Option<String>,
    pub guid: Guid,
    pub modified: String,
    pub modified_gmt: String,
    pub slug: String,
    pub status: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub link: String,
    pub title: Title,
    pub content: Content,
    pub excerpt: Excerpt,
    pub author: i64,
    pub featured_media: i64,
    pub comment_status: String,
    pub ping_status: String,
    pub sticky: bool,
    pub template: String,
    pub format: String,
    pub meta: Meta,
    pub categories: Vec<i64>,
    pub tags: Vec<i64>,
    pub yoast_head: String,
    pub yoast_head_json: Value,
    pub _links: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    pub rendered: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub id: Value,
    pub date: Option<String>,
    pub date_gmt: Option<String>,
    pub guid: Guid,
    pub modified: String,
    pub modified_gmt: String,
    pub slug: String,
    pub status: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub link: String,
    pub title: Title,
    pub author: i64,
    pub comment_status: String,
    pub ping_status: String,
    pub template: String,
    pub meta: Meta,
    pub description: Value,//Description,
    pub caption: Value,
    pub alt_text: String,
    pub media_type: String,
    pub mime_type: String,
    pub media_details: Value,
    pub post: Value,
    pub source_url: String,
    pub _links: Value,
}

impl Media {
    pub fn from(uri: &str, id: String) -> Option<Media> {
        println!("{}", &format!("{}/wp-json/wp/v2/media/{}", uri, id));
        //println!("{:?}", &ureq::get(&format!("{}/wp-json/wp/v2/media", id)).call().unwrap().into_string().unwrap());
        let mut response = String::new();
        if cache::media::has(id.clone()) {
            response = cache::media::get(id);
        } else {
            response = ureq::get(&format!("{}/wp-json/wp/v2/media/{}", uri, id)).call().unwrap().into_string().unwrap();
            cache::media::add(id, response.clone())
        }

        serde_json::from_str(&response).unwrap()
    }
}

impl Post {
    pub fn get_from_uri(uri: &str) -> Option<Vec<Post>> {
        /*match &ureq::get(&format!("{}/wp-json/wp/v2/posts", uri)).call() {
            Ok(mut data) => {
                match data.into_string() {
                    Ok(data1) => {
                        match serde_json::from_str(&data1) {
                            Ok(data2) => Some(data2),
                            Err(_) => None
                        }
                    },
                    Err(_) => None,
                }
            },
            Err(_) => None
        }*/
        
        serde_json::from_str(&ureq::get(&format!("{}/wp-json/wp/v2/posts", uri)).call().unwrap().into_string().unwrap()).unwrap()
    }
    pub fn get_from_uri_limited(uri: &str, per_page: i64) -> Option<Vec<Post>> {
        let mut response = String::new();
        if cache::post::has() {
            response = cache::post::get();
        } else {
            response = ureq::get(&format!("{}/wp-json/wp/v2/posts?per_page={}", uri, per_page)).call().unwrap().into_string().unwrap();
            cache::post::add(response.clone());
        }

        serde_json::from_str(&response).unwrap()
    }
}

pub fn get_from_uri_by_tag(uri: &str, tag: i64) -> Result<Vec<Post>> {
    let mut result: Vec<Post> = Vec::new();
    let data: Vec<Post> = Post::get_from_uri(uri).unwrap();
    for post in data.clone().iter() {
        if post.tags.contains(&tag) {
            result.push((*post).clone())
        }
    }
    Ok(result)
}

pub fn get_from_uri_by_category(uri: &str, category: i64) -> Result<Vec<Post>> {
    let mut result: Vec<Post> = Vec::new();
    let data: Vec<Post> = Post::get_from_uri(uri).unwrap();
    for post in data.clone().iter() {
        if post.categories.contains(&category) {
            result.push((*post).clone())
        }
    }
    Ok(result)
}


