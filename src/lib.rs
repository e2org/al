extern crate rand;

use rand::seq::SliceRandom;
use std::error;
use std::fmt;

// Standard "error-boxing" Result type:
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Args {
    pub alias: String,
    pub edit: Option<String>,
    pub rename: Option<String>,
    pub delete: bool,
    pub color: Option<String>,
    pub verbose: bool,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ARGS: alias={} edit={} rename={} delete={} color={} verbose={}",
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
            match &self.color {
                Some(arg) => format!("\"{}\"", arg),
                None => String::from("None"),
            },
            self.verbose,
        )
    }
}

impl Args {
    pub fn new(matches: clap::ArgMatches) -> Result<Args> {
        let alias = String::from(matches.value_of("ALIAS").unwrap_or(""));
        let edit = matches.value_of("EDIT").map(String::from);
        let rename = matches.value_of("RENAME").map(String::from);
        let delete = matches.is_present("delete");
        let color = matches.value_of("color").map(String::from);
        let verbose = matches.is_present("verbose");
        Ok(Args {
            alias,
            edit,
            rename,
            delete,
            color,
            verbose,
        })
    }
}

pub fn colorscheme(args: &Args) -> &'static str {
    let colorschemes = vec![
        // Colorscheme CMYK:
        "bg+:-1,border:#0000ff,pointer:#0bc7e3,prompt:#feaf3c,\
            info:#0000ff,fg:#0000ff,fg+:#0bc7e3,hl:#ff00ff,hl+:#ff00ff",
        // Colorscheme Outrun:
        "bg+:-1,border:#541388,pointer:#ef2b63,prompt:#0bc7e3,\
            info:#541388,fg:#541388,fg+:#ef2b63,hl:#0bc7e3,hl+:#0bc7e3",
        // Colorscheme Submariner:
        "bg+:-1,border:#1d485f,pointer:#0bc7e3,prompt:#db662d,\
            info:#1d485f,fg:#1d485f,fg+:#0bc7e3,hl:#db662d,hl+:#db662d",
    ];

    // Return colorscheme matching provided --color arg:
    match args.color.as_deref().unwrap_or("") {
        "cmyk" | "c" => colorschemes[0],
        "outrun" | "o" => colorschemes[1],
        "submariner" | "s" => colorschemes[2],
        "random" | "r" => colorschemes.choose(&mut rand::thread_rng()).unwrap(),
        _ => colorschemes[2], // default Submariner
    }
}
