use std::fs::File;

use rocket::{Build, Rocket, State, fs::{FileServer, NamedFile}};
use clap::Parser;

#[macro_use] extern crate rocket;


/// I'm a rocket aaaapp
/// Rocket aaaaapp!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WConfig {
    /// Directory where the website files are found
    site_path: String,

    /// The filepath where the list of blogs (index.json) can be found
    blog_root_path: String,

    // md, html
    //allowed_types: Vec<String>,
}

#[rocket::main]
async fn main() {
    let config = WConfig::parse();
    let _ = waffsite(config).launch().await;
}

fn waffsite(config : WConfig) -> Rocket<Build> {
    let dir = config.site_path.to_string();
    let content = config.blog_root_path.to_string();

    // add rocket_cors or alternative if I end up switching to two domains for front/back

    if let Err(_) = File::open(format!("{}/blogs.json",config.blog_root_path)) {
        panic!("Couldn't read `blogs.json` in the provided root directory.
Does it exist? Does the process have read-permission to it?")
    }

    rocket::build()
        .manage(config)
        .mount("/", FileServer::from(dir))
        .mount("/blog", routes![bloglist])
        .mount("/blog/data", FileServer::from(content).rank(-1))
}


#[get("/list")]
 async fn bloglist(config : &State<WConfig>) -> Option<NamedFile> {
    NamedFile::open(format!("{}/blogs.json",config.blog_root_path)).await.ok()
}

