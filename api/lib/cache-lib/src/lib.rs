pub mod media {
    pub fn has(id: String) -> bool {
        use std::path::Path;
        println!("checking for cached version of: {}", id);
        Path::new(&format!("cache/media/{}", id)).exists()
    }

    pub fn add(id: String, value: String) {
        use std::fs::File;
        use std::fs;
        File::create(format!("cache/media/{}", id)).unwrap();
        fs::write(format!("cache/media/{}", id), value).expect("Unable to write file");
    }

    pub fn get(id: String) -> String {
        use std::fs;
        println!("getting for cached version of: {}", id);
        fs::read_to_string(format!("cache/media/{}", id)).expect("Unable to read file")
    }
}

pub mod post {
    use chrono::Local;
    pub fn has() -> bool {
        let id = Local::now().format("%Y-%m-%d").to_string();
        use std::path::Path;
        println!("checking for cached version of: {} -> {}", id, Path::new(&format!("cache/posts/{}", id)).exists());
        Path::new(&format!("cache/posts/{}", id)).exists()
    }

    pub fn add(value: String) {
        let id = Local::now().format("%Y-%m-%d").to_string();
        use std::fs::File;
        use std::fs;
        println!("adding cached version of: {}", id);
        File::create(format!("cache/posts/{}", id)).unwrap();
        fs::write(format!("cache/posts/{}", id), value.clone()).expect("Unable to write file");
        fs::write("cache/posts/fallback", value).expect("Unable to write file");
    }

    pub fn get() -> String {
        let id = Local::now().format("%Y-%m-%d").to_string();
        use std::fs;
        println!("getting for cached version of: {}", id);
        fs::read_to_string(format!("cache/posts/{}", id)).expect("Unable to read file")
    }

    pub fn refresh() {
        let id = Local::now().format("%Y-%m-%d").to_string();
        use std::fs;
        match fs::remove_file(format!("cache/posts/{}", id)) {
            _ => {}
        }
    }
}