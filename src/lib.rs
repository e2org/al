use std::error;
use std::fmt;

// Standard "error-boxing" Result type:
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Args {
    pub alias: String,
    pub edit: Option<String>,
    pub rename: Option<String>,
    pub delete: bool,
    pub verbose: bool,
    pub quiet: bool,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ARGS: alias={} edit={} rename={} delete={} verbose={} quiet={}",
            format!("\"{}\"", self.alias),
            match &self.edit {
                Some(arg) => format!("\"{}\"", arg),
                None => String::from("None"),
            },
            match &self.rename {
                Some(arg) => format!("\"{}\"", arg),
                None => String::from("None"),
            },
            self.delete,
            self.verbose,
            self.quiet
        )
    }
}

impl Args {
    pub fn new(matches: clap::ArgMatches) -> Result<Args> {
        let alias = String::from(matches.value_of("ALIAS").unwrap_or_else(|| ""));
        let edit = matches.value_of("EDIT").map(|arg| String::from(arg));
        let rename = matches.value_of("RENAME").map(|arg| String::from(arg));
        let delete = matches.is_present("delete");
        let verbose = matches.is_present("verbose");
        let quiet = matches.is_present("quiet");
        Ok(Args {
            alias,
            edit,
            rename,
            delete,
            verbose,
            quiet,
        })
    }
}
