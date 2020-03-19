#![feature(proc_macro_hygiene, decl_macro)]

mod cli;
mod routes;

use cli::Cli;

use std::env;

use rocket::{catchers, routes};
use structopt::StructOpt;

pub static INDEX: &str = include_str!("www/index.html");
pub static CSS: &str = include_str!("www/style.css");
pub static POST: &str = include_str!("www/post.html");
pub static FAVICON: &'static [u8] = include_bytes!("www/favicon.ico");

fn main() {
    let opt = Cli::from_args();

    // port setting
    env::set_var("ROCKET_PORT", format!("{}", opt.port));

    // rocket server init
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::css,
                routes::rules,
                routes::post,
                routes::favicon
            ],
        )
        .register(catchers![routes::not_found])
        .launch();
}
