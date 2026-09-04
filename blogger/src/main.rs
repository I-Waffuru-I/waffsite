use rocket::{Build, Rocket, State, fs::NamedFile};
use clap::Parser;
use std::path;

#[macro_use] extern crate rocket;


/// just an app man
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WConfig {
    /// Directory where the root files are found
    root_dir: String,

    /// The filepath where the list of blogs can be found
    list_path: String,

    /// Directory where the blogs are found
    list_dir: String,
}


#[rocket::main]
async fn main() {
    let config = WConfig::parse();
    let _ = waffsite(config).launch().await;
}

fn waffsite(config : WConfig) -> Rocket<Build> {
    rocket::build()
        .manage(config)
        .mount("/", routes![index])
        .mount("/blog", routes![bloglist, get_blog])
}

#[get("/")]
fn index() -> String {
    "page".to_string()
}

#[get("/list")]
 async fn bloglist(config : &State<WConfig>) -> Result<NamedFile, String> {
    NamedFile::open(path::Path::new(&config.list_path))
    .await.map_err(|_| "Can't find blogs.txt".to_string() )
}

#[get("/byid/<id>")]
fn get_blog(id: &str) -> String {
    format!("blog {id}")
}
