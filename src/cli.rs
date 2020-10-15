use structopt::StructOpt;

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cli {
	/// port
	#[structopt(short, long, default_value = "8000")]
	pub port: u16,

	/// max capacity of saved files in GB
	#[structopt(short, long)]
	pub capacity: Option<u16>,
}
