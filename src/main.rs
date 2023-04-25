#[macro_use]
extern crate rocket;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use percentages::run;

#[derive(Serialize, Debug)]
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
    let mut percentages_map: HashMap<String, f64> = HashMap::new();
    let mut error_map: HashMap<String, TemplateErrorContext> = HashMap::new();
    let mut seen_errors: HashSet<String> = HashSet::new();
    let mut index: u8 = 1;
    let mut indexes: Vec<u8> = Vec::from([index]);

    for equation in equations {
        match run(equation) {
            Ok(result) => {
                percentages_map.insert(index.to_string(), result.percentage);
                index += 1;
            }
            Err(error_data) => {
                let error: String = error_data.error;
                let emphasis: Option<&str> = error_data.emphasis;

                // See if error has been seen before. If so, push to indexes.
                if error_map.is_empty() {
                    seen_errors.insert(error.clone());
                } else {
                    for value in error_map.values() {
                        if !seen_errors.insert(value.error.clone()) {
                            // Get the index of an error instance and remove it.
                            let position = error_map
                                .iter()
                                .position(|v| v.1.error == value.error.clone());
                            error_map.remove(&position.unwrap());
                            indexes.push(index);
                        }
                    }
                }

                //println!("{:?}", seen_errors);

                // Using itertools as Rust won't stringify a Vec<u8> concatenation.
                let split: String = indexes.iter().join(", ");

                error_map.insert(split, TemplateErrorContext { error, emphasis });
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
