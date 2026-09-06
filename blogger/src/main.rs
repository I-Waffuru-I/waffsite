use rocket::{Build, Rocket, State, fs::{FileServer, NamedFile}, response::content::RawJson};
use clap::Parser;
use rocket_cors::{AllowedOrigins, CorsOptions};

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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true)
        .to_cors().expect("Failed to create cors ");

    rocket::build()
        .manage(config)
        .attach(cors)
        .mount("/", FileServer::from(dir))
        .mount("/blog", routes![bloglist])
        .mount("/blog/data", FileServer::from(content).rank(-1))
}


#[get("/list")]
 async fn bloglist(config : &State<WConfig>) -> Option<NamedFile> {
    NamedFile::open(format!("{}/blogs.json",config.blog_root_path)).await.ok()
}

