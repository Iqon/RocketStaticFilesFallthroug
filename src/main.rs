use rocket::ignite;
use rocket::local::Client;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

fn server() -> Rocket {
    ignite()
        .mount("/static", StaticFiles::from("./folder1").rank(1))
        .mount("/static", StaticFiles::from("./folder2").rank(2))
}

fn main() {
    server().launch();
}

#[test]
fn get_the_first_file() {
    let server = Client::new(server()).unwrap();
    let mut request = server.get("/static/file1.txt").dispatch();
    let body = request.body_string().unwrap();
    assert_eq!(body, "hi from file 1");
}

#[test]
fn get_the_second_file() {
    let server = Client::new(server()).unwrap();
    let mut request = server.get("/static/file2.txt").dispatch();
    let body = request.body_string().unwrap();
    assert_eq!(body, "hi from file 2");
}
