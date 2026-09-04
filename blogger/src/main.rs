#[macro_use] extern crate rocket;



#[launch]
fn waffsite() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/blog", routes![bloglist, get_blog])
}

#[get("/")]
fn index() -> String {
    "page".to_string()
}

#[get("/list")]
fn bloglist() -> String {
    let contents = std::fs::read_to_string("./blogs.txt");

    match contents {
        Ok(s) => {s}
        Err(e) => {todo!()} 
    }
}

#[get("/byid/<id>")]
fn get_blog(id: &str) -> String {
    format!("blog {id}")
}
