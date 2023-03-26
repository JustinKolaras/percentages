#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use percentages::run;

#[derive(FromForm)]
struct Equation {
    equation: String,
}

/// GET.

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("static/home.html").await.ok()
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

    match run(eq.to_string()) {
        Ok(result) => Template::render(
            "success",
            context! {
                elements: result.elements,
                percentage: result.percentage
            },
        ),
        Err(err) => Template::render(
            "error",
            context! {
                error: err.error,
                emphasis: err.emphasis
            },
        ),
    }
}

/// Catchers.

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open("static/catchers/404.html").await.ok()
}

#[catch(500)]
async fn internal_error() -> Option<NamedFile> {
    NamedFile::open("static/catchers/500.html").await.ok()
}

/// Launch.

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, internal_error])
        .mount("/", routes![home, results, style])
        .attach(Template::fairing())
}
