#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::request::Request;
use rocket_dyn_templates::{context, Template};

use percentages::run;

#[derive(FromForm)]
struct Equation {
    equation: String,
}

/// GET.

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

// Manually add a route to return the CSS file.
#[get("/style.css")]
async fn style() -> Option<NamedFile> {
    NamedFile::open("static/style.css").await.ok()
}

/// POST.

#[post("/results", data = "<eq>")]
async fn results(eq: Form<Equation>) -> Template {
    let eq: Equation = eq.into_inner();
    let eq: String = eq.equation;
    let eq: &str = eq.trim();

    let results: String = match run(eq.to_string()) {
        Ok(result) => format!("Elements: {}\nResult: {}%", result.elements, result.result),
        Err(err) => format!("Computation error.\n\n{}", err),
    };

    Template::render(
        "results",
        context! {
            results,
        },
    )
}

/// Catchers.

#[catch(404)]
fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, `{}` is not a valid path.", req.uri())
}

/// Launch.

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![home, results, style])
        .attach(Template::fairing())
}
