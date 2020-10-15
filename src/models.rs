use rocket::request::FromForm;

#[derive(Debug, FromForm)]
pub struct Post {
	pub content: String,
}
