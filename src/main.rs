#[macro_use]
extern crate rocket;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use percentages::run;
use percentages::ErrorData;

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

    // Most of these data sets use the equation number as their key
    // to circumvent HBS issues (@key as replacement for unprogrammable @index).
    let mut percentages_map: HashMap<String, f64> = HashMap::new();
    let mut error_map: HashMap<String, ErrorData> = HashMap::new();
    // Need a new HashMap to deal with mutability issues. Again, will probably refactor later.
    let mut parsed_error_map: HashMap<String, ErrorData> = HashMap::new();
    let mut seen_errors: HashSet<String> = HashSet::new();
    //let mut error_remove_pile: Vec<&String> = Vec::new();
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
                let error_id: String = error_data.id;

                // See if error has been seen before. If so, push to indexes.
                if error_map.is_empty() {
                    seen_errors.insert(error_id);
                } else {
                    for (key, value) in error_map.iter() {
                        let error: String = value.id.clone();
                        /*
                        if !seen_errors.insert(error) {
                            // Push the index. Removing the previous error is done later (see comment).
                            indexes.push(index);

                            // This is horribly sloppy, but due to constrictions with Rust's
                            // borrow checker, I can't figure out a better way to do this.
                            // Will probably refactor later.
                            //
                            // All errors will be pushed to error_remove_pile, then, outside the loop,
                            // I'll loop through this vector and remove the corresponding keys.
                            error_remove_pile.push(key);
                        }
                        */
                        
                        // Try another approach in which the successors are pushed to a 
                        // new vector.

                        let error_clone: String = error.clone();
                        let id_clone: String = error_id.clone();

                        if !seen_errors.insert(error) {
                            indexes.push(index);
                            continue;
                        }

                        parsed_error_map.insert(index.to_string(), ErrorData { error: error_clone, emphasis, id: id_clone });
                    }
                }

                /*
                for key in error_remove_pile.iter() {
                    error_map.remove(*key);
                }
                */

                // Using itertools as Rust won't stringify a Vec<u8> concatenation.
                let split: String = indexes.iter().join(", ");

                error_map.insert(split, ErrorData { error, emphasis, id: error_id });
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
