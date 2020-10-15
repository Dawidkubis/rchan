use crate::models::Post;
use crate::{CSS, FAVICON, INDEX, POST, README};

use rocket::http::ContentType;
use rocket::request::Form;
use rocket::response::content::{Content, Css, Html};
use rocket::{catch, get, post, Request};

// the gets

/// index path
#[get("/")]
pub fn index() -> Html<String> {
	Html(INDEX.to_owned())
}

#[get("/<id>")]
pub fn thread(id: usize) -> String {
	"nigger".to_owned()
}

/// css, for markdown stylization purposes
#[get("/css")]
pub fn css() -> Css<String> {
	Css(CSS.to_owned())
}

/// favicon
#[get("/favicon.ico")]
pub fn favicon() -> Content<&'static [u8]> {
	Content(ContentType::Icon, FAVICON)
}

/// readme page
#[get("/readme")]
pub fn rules() -> Html<String> {
	Html(README.to_owned())
}

/// post form
#[get("/post")]
pub fn post() -> Html<String> {
	Html(POST.to_owned())
}

// the posts TODO

#[post("/post", data = "<post>")]
pub fn post_post(post: Form<Post>) -> String {
	format!("{:?}", post)
}

// the catchers
/// catch 404
#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
