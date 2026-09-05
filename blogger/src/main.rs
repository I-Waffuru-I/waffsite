use rocket::{Build, Rocket, State, fs::{FileServer, NamedFile}};
use clap::Parser;
use std::path;

#[macro_use] extern crate rocket;


/// I'm a rocket aaaap
/// Rocket aaaaapp!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WConfig {
    /// Directory where the root files are found
    root_path: String,

    /// The filepath where the list of blogs can be found
    list_path: String,
    // md, html
    //allowed_types: Vec<String>,
}

// Managed lijst blog files in 
#[derive(Debug)]
struct CachedBlogList {
    pub list: Vec<String>
}


#[rocket::main]
async fn main() {
    let config = WConfig::parse();
    let _ = waffsite(config).launch().await;
}

fn waffsite(config : WConfig) -> Rocket<Build> {
    let dir = config.root_path.to_string();

    // maak cached lijst van blogs in de dir
    let mut cache = CachedBlogList { list : vec!() };
    if let Ok(d) = std::fs::read_dir(config.list_path.to_string()) {
        let entries = d.filter_map(|item| item.ok());
        let s = entries.filter_map(|e| {
            if e.file_type().is_ok_and(|ft| ft.is_file()) {
                e.file_name().into_string().ok()
            } else {
                None
            }
        });
        for file in s {
            cache.list.push(file);
        }
    };

    rocket::build()
        .manage(config)
        .manage(cache)
        //.mount("/", routes![index])
        .mount("/", FileServer::from(dir))
        .mount("/blog", routes![bloglist, get_blog])
}


#[get("/list")]
 async fn bloglist(cache : &State<CachedBlogList>) -> String {
    cache.list.join(";")
}

#[get("/byid/<id>")]
fn get_blog(config: &State<WConfig>, id: &str) -> String {
    format!("blog {}/{}",config.list_path,id)
}
