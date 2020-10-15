use std::fs::{read_to_string, write};

use comrak::{markdown_to_html, ComrakOptions};

fn main() {
	let mut skeleton = read_to_string("src/www/index.html").unwrap();
	let readme = read_to_string("README.md").unwrap();

	skeleton = skeleton
		.replace("{name}", "readme")
		.replace("{}", &markdown_to_html(&readme, &ComrakOptions::default()));

	write("src/www/readme.html", skeleton).unwrap();
}
