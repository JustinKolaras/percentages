#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use percentages::run;

#[derive(FromForm)]
struct Equation {
    equations: Vec<String>,
}

/// GET.

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("static/home.html").await.ok()
}

// Manually added routes.
#[get("/style.css")]
async fn style() -> Option<NamedFile> {
    NamedFile::open("styles/style.css").await.ok()
}

#[get("/homeLogic.js")]
async fn home_logic() -> Option<NamedFile> {
    NamedFile::open("scripts/homeLogic.js").await.ok()
}

/// POST.

#[post("/results", data = "<equations>")]
async fn results(equations: Form<Equation>) -> Template {
    let equations: Equation = equations.into_inner();
    let equations: Vec<String> = equations.equations;

    let mut to_send_elements: Vec<u64> = Vec::new();
    let mut to_send_percentages: Vec<f64> = Vec::new();

    for equation in equations {
        match run(equation) {
            Ok(result) => {
                to_send_elements.push(result.elements);
                to_send_percentages.push(result.percentage);
            }
            Err(error) => {
                return Template::render(
                    "error",
                    context! {
                        error: error.error,
                        emphasis: error.emphasis
                    },
                )
            }
        }
    }

    Template::render(
        "success",
        context! {
            elements: to_send_elements,
            percentage: to_send_percentages
        },
    )
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
        .mount("/", routes![home, results, style, home_logic])
        .attach(Template::fairing())
}
