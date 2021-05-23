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
                Some(val) => format!("\"{}\"", val),
                None => "None".to_string(),
            },
            match &self.rename {
                Some(val) => format!("\"{}\"", val),
                None => "None".to_string(),
            },
            self.delete,
            self.verbose,
            self.quiet
        )
    }
}

impl Args {
    pub fn new(matches: clap::ArgMatches) -> Result<Args> {
        let alias = match matches.value_of("ALIAS") {
            Some(arg) => arg.to_string(),
            None => "".to_string(),
        };
        let edit = match matches.value_of("EDIT") {
            Some(arg) => Some(arg.to_string()),
            None => None,
        };
        let rename = match matches.value_of("RENAME") {
            Some(arg) => Some(arg.to_string()),
            None => None,
        };
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
