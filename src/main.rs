extern crate clap;
extern crate regex;
extern crate skim;

use clap::clap_app;
use regex::RegexBuilder;
use skim::prelude::{Skim, SkimItemReader, SkimOptionsBuilder};
use std::io::Cursor;
use std::process::{self, Command, Stdio};

use al::{colorscheme, Args};

fn main() {
    let args = Args::new(
        clap_app!(al =>
            (version: env!("CARGO_PKG_VERSION"))
            (author: env!("CARGO_PKG_AUTHORS"))
            (about: env!("CARGO_PKG_DESCRIPTION"))
            (@arg ALIAS:
             "alias to execute or operate on")
            (@arg EDIT: -e --edit +takes_value
             "create or edit alias value")
            (@arg MOVE: -m --move +takes_value
             "change alias name")
            (@arg delete: -d --delete
             "continuously output images to terminal")
            (@arg color: -c --color +takes_value
             "choose color scheme")
            (@arg verbose: -v --verbose
             "output info/debugging output to terminal")
        )
        .get_matches(),
    )
    .unwrap_or_else(|error| panic!("error: {:?}", error));

    if args.verbose {
        println!("{}", args);
    }

    // Get output of "alias" command for the user's local shell:
    let output = Command::new("bash")
        .arg("-c") // execute in user shell so user-defined aliases are available
        .arg("-i") // shell must be "interactive"
        .arg("alias")
        .output()
        .expect("Failed to list aliases.");
    let aliases = String::from(String::from_utf8_lossy(&output.stdout));
    let preview = format!("grep {{q}} <<EOF\n{}\nEOF", aliases);

    // Format each item for display via Skim:
    // "alias foo='bar'" --> "foo"
    let re = RegexBuilder::new(r"((=[^\n]+)($|\n))?(^|\n)(alias )?")
        .multi_line(true)
        .build()
        .unwrap();
    let items = SkimItemReader::default()
        .of_bufread(Cursor::new(String::from(re.replace_all(&aliases, "\n"))));

    // Run Skim to allow user to select alias:
    let result = Skim::run_with(
        &SkimOptionsBuilder::default()
            .query(Some(&args.alias))
            .color(Some(colorscheme(&args)))
            .prompt(Some("$ "))
            .margin(Some("1,2"))
            .height(Some("40%"))
            .reverse(true)
            .inline_info(true)
            .preview(Some(&preview)) // show full alias text alongside each item
            // {{q}} refers to current query string
            .build()
            .unwrap(),
        Some(items),
    )
    .unwrap();

    if result.is_abort {
        process::exit(0);
    }

    // If no alias is selected, use query string:
    let mut choice = result.query;
    for item in result.selected_items.iter() {
        choice = String::from(item.output());
    }

    // Output selection, matching text alignment with skim prompt:
    println!("\n  $ {}\n", choice);

    // Execute selected alias:
    Command::new("bash")
        .arg("-c") // execute in user shell so user-defined aliases are available
        .arg("-i") // shell must be "interactive"
        .arg(choice)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute alias.");
}
