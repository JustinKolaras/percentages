#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use percentages::run;

#[derive(Serialize)]
struct TemplateErrorContext {
    error: String,
    emphasis: Option<&'static str>,
}

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

#[get("/logic.js")]
async fn logic() -> Option<NamedFile> {
    NamedFile::open("scripts/logic.js").await.ok()
}

/// POST.

#[post("/results", data = "<equations>")]
async fn results(equations: Form<Equation>) -> Template {
    let equations: Equation = equations.into_inner();
    let equations: Vec<String> = equations.equations;

    // Impromptu solution to HBS issues by using @key & not @index.
    // Might find another solution later.
    let mut percentages_map: HashMap<String, f64> = HashMap::new();
    let mut error_map: HashMap<String, TemplateErrorContext> = HashMap::new();
    let mut index: u8 = 1;

    for equation in equations {
        match run(equation) {
            Ok(result) => {
                percentages_map.insert(index.to_string(), result.percentage);
                index += 1;
            }
            Err(error) => {
                error_map.insert(
                    index.to_string(),
                    TemplateErrorContext {
                        error: error.error,
                        emphasis: error.emphasis,
                    },
                );
                index += 1;
            }
        }
    }

    if !error_map.is_empty() {
        return Template::render(
            "error",
            context! {
                error: error_map,
            },
        );
    }

    Template::render(
        "success",
        context! {
            percentage: percentages_map,
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
        .mount("/", routes![home, results, style, logic])
        .attach(Template::fairing())
}
