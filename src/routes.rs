use crate::{CSS, FAVICON, INDEX, POST};

use rocket::http::ContentType;
use rocket::response::content::{Content, Css, Html};
use rocket::{catch, get, Request};

/// index path
#[get("/")]
pub fn index() -> Html<String> {
    Html(INDEX.to_owned())
}

/// css, for markdown stylization purposes
#[get("/css")]
pub fn css() -> Css<String> {
    Css(CSS.to_owned())
}

#[get("/favicon.ico")]
pub fn favicon() -> Content<&'static [u8]> {
    Content(ContentType::Icon, FAVICON)
}

/// readme page
#[get("/readme")]
pub fn rules() -> String {
    "Still thinking about what to put here".to_owned()
}

/// post
#[get("/post")]
pub fn post() -> Html<String> {
    Html(POST.to_owned())
}

// catchers
/// catch 404
#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("404: {} is not a valid path", req.uri())
}
