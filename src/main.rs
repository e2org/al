extern crate clap;
extern crate rand;
extern crate skim;

use clap::clap_app;
use rand::seq::SliceRandom;
use skim::prelude::{Skim, SkimItemReader, SkimOptionsBuilder};
use std::io::Cursor;
use std::process::Command;

use al::Args;

fn main() {
    let args = Args::new(
        clap_app!(al =>
            (version: env!("CARGO_PKG_VERSION"))
            (author: env!("CARGO_PKG_AUTHORS"))
            (about: env!("CARGO_PKG_DESCRIPTION"))
            (@arg ALIAS: "alias to execute or operate on")
            (@arg EDIT: -e --edit +takes_value "create or edit alias value")
            (@arg MOVE: -m --move +takes_value "change alias name")
            (@arg delete: -d --delete "continuously output images to terminal")
            (@arg verbose: -v --verbose "output info/debugging output to terminal")
            (@arg quiet: -q --quiet "suppress all output -- run silently")
        )
        .get_matches(),
    )
    .unwrap_or_else(|error| panic!("error: {:?}", error));

    if args.verbose {
        println!("{}", args);
    }

    let alias_cmd = "source $HOME/.aliases; source $HOME/.bash_aliases; alias";
    let format_cmd = "| cut -d'=' -f1 | cut -d' ' -f2";
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("{} {}", alias_cmd, format_cmd))
        .output()
        .expect("Failed to list aliases.");

    let aliases = String::from(String::from_utf8_lossy(&output.stdout));

    let items = SkimItemReader::default().of_bufread(Cursor::new(aliases));

    let colorschemes = vec![
        // Colorscheme CMYK:
        "bg+:-1,border:#0000ff,pointer:#0bc7e3,prompt:#feaf3c,info:#0000ff,fg:#0000ff,fg+:#0bc7e3,hl:#ff00ff,hl+:#ff00ff",
        // Colorscheme Outrun:
        "bg+:-1,border:#541388,pointer:#ef2b63,prompt:#0bc7e3,info:#541388,fg:#541388,fg+:#ef2b63,hl:#0bc7e3,hl+:#0bc7e3",
        // Colorscheme Submariner:
        "bg+:-1,border:#1d485f,pointer:#0bc7e3,prompt:#db662d,info:#1d485f,fg:#1d485f,fg+:#0bc7e3,hl:#db662d,hl+:#db662d",
    ];
    let colorscheme = colorschemes.choose(&mut rand::thread_rng()).unwrap();

    let selected = Skim::run_with(
        &SkimOptionsBuilder::default()
            .query(Some(&args.alias))
            .color(Some(colorscheme))
            .prompt(Some("$ "))
            .margin(Some("1,2"))
            .height(Some("40%"))
            .preview(Some(&format!("bash -c '{}' | grep {{q}}", alias_cmd)))
            // {{q}} refers to current query string
            .reverse(true)
            .build()
            .unwrap(),
        Some(items),
    )
    .map(|out| out.selected_items)
    .unwrap_or_else(Vec::new);

    println!();
    for item in selected.iter() {
        println!("  $ {}", item.output());
    }
    println!();
}
