use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the S3 file to be processed
    pub url: Url,

    #[clap(short, long, default_value_t = String::from("select * from tbl"), group = "sql")]
    /// SQL query running against the file
    pub query: String,

    #[clap(short = 'p', long = "profile")]
    /// AWS profile to use for credentials, support assuming roles
    pub aws_profile: Option<String>
}

impl Args {
    pub fn parse() -> Self {
        Args::parse_from(&mut std::env::args())
    }
}
