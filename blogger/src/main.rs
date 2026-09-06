use rocket::{Build, Rocket, State, fs::FileServer};
use clap::Parser;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use] extern crate rocket;


/// I'm a rocket aaaapp
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

#[rocket::main]
async fn main() {
    let config = WConfig::parse();
    let _ = waffsite(config).launch().await;
}

fn waffsite(config : WConfig) -> Rocket<Build> {
    let dir = config.root_path.to_string();
    let content = config.list_path.to_string();

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
 async fn bloglist(config : &State<WConfig>) -> String {
    let mut cache = vec!();
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
            if file.ends_with(".md") {
                cache.push(file);
            }
        }
    };
    cache.join(";")
}

