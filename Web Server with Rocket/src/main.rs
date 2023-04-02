use rocket::serde::json::Json;
use rocket::{launch, routes, get, post};
use rocket::serde::{Serialize, Deserialize};
use rocket::response::content;
use std::env;
use std::fs;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

// TODO 1 - write a route that reverses the provided string
// - the string will be provided within the route

#[get("/reverse/<string>")]
fn reverse(string: &str) -> String {
    string.chars().rev().collect()
}

// TODO 2 - write a route that reverses a string and 
// return a JSON with it
// Hint use a structure that has a String and Serde

#[get("/reverse_json/<string>")]
fn reverse_json(string: &str) -> Json<WJSON> {
    let json = WJSON {
        str: string.chars().rev().collect(),
    };
    Json(json)
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct WJSON {
    str: String,
}

// TODO 3 - write a route that receives String in a JSON
// and returns it reversed in a JSON (use post)

#[post("/reverse_json_post", data = "<jsn_string>")]
fn reverse_json_post(jsn_string: Json<WJSON>) -> Json<WJSON> {
    let rev = jsn_string.str.chars().rev().collect();
    let new_json = WJSON {
        str: rev,
    };
    Json(new_json)
}

// TODO 4 - write a route that returns the list of files
// in the current directory
#[get("/list_files")]
fn list_files() -> String {
    let mut files = Vec::new();
    let dir = env::current_dir().unwrap();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        files.push(file_name.to_string());
    }
    files.join("\n")   
}


// TODO 5 - write a route that returns the list of files
// from a received path that is a subpath of a directory
// recived in the command line
//
// Hint use std::env

#[get("/list_files_sub/<path>")]
fn list_files_sub(path: &str) -> String {
    let mut files = Vec::new();
    let mut dir = env::current_dir().unwrap();
    dir.push(path);
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        files.push(file_name.to_string());
    }
    files.join("\n")   
}

#[get("/list_directories")]
fn list_directories() -> content::RawHtml<String> {
    let mut dirs = Vec::new();
    let dir = env::current_dir().unwrap();
    let uri = "https://horiamoraru-obscure-capybara-rvj65pqjqj42pvqp-8000.preview.app.github.dev/";
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if path.is_dir() {
            let link = format!("{} {} {}", uri, "list_files_sub/", file_name);
            dirs.push(format!("<a href=\"{link}\"</a>", link = link));
        }
    }
    content::RawHtml(dirs.join("\n"))   
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world, reverse, reverse_json, reverse_json_post, list_files, list_files_sub, list_directories])
}

/* alternative
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await?;

    Ok(())
}
*/