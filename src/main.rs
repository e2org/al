extern crate clap;

use clap::clap_app;

use al::Args;

// Get config values directly from Cargo.toml so they _never_ get out of sync:
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    // Parse arguments from command line via https://github.com/clap-rs/clap
    let args = Args::new(
        clap_app!(al =>
            // Use config values from Cargo.toml:
            (version: VERSION)
            (author: AUTHOR)
            (about: DESCRIPTION)
            // Positional argument:
            (@arg ALIAS: "alias to execute or operate on")
            // Keyword arguments:
            (@arg EDIT: -e --edit +takes_value "create or edit alias value")
            (@arg MOVE: -m --move +takes_value "change alias name")
            // Boolean arguments (flags):
            (@arg delete: -d --delete "continuously output images to terminal")
            (@arg verbose: -v --verbose "output info/debugging output to terminal")
            (@arg quiet: -q --quiet "suppress all output -- run silently")
        )
        // Args constructor accepts a clap::ArgMatches object:
        .get_matches(),
    )
    // Args constructor will error if no target directory was provided and
    // it is unable to determine current working directory of script.
    // In this case, print the error and exit immediately (!panic):
    .unwrap_or_else(|error| panic!("error: {:?}", error));

    // If verbose mode requested, print info line with argument values.
    // Formatting handled via Args::fmt -- implementation of Display trait.
    if args.verbose {
        println!("{}", args);
    }
}
