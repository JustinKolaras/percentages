#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::request::Request;

use percentages::{run, CalcResult};

#[derive(FromForm)]
struct Equation {
    equation: String,
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[post("/submit", data = "<eq>")]
async fn submit(eq: Form<Equation>) -> String {
    let eq: Equation = eq.into_inner();
    let eq: String = eq.equation;
    let eq: &str = eq.trim();

    let result: Result<CalcResult, String> = run(eq.to_string());

    match result {
        Ok(result) => format!("Elements: {}\nResult: {}", result.elements, result.result),
        Err(err) => err,
    }
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, `{}` is not a valid path.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
        .register("/", catchers![not_found])
}
