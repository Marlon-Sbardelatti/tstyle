use core::panic;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut hex = String::new();
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <theme_name>", args[0]);
        std::process::exit(1);
    }

    // Extract the theme name from the arguments
    let theme_name = &args[1];
    if theme_name == "ayu" {
        hex = String::from("#0a0e14");
    } else if theme_name == "tkn" {
        hex = String::from("#11121d");
    } else {
        panic!("Invalid arg, please provide a valid colorscheme name");
    }

    let file = File::open("/home/hetzwga/.tmux/plugins/tmux/catppuccin-mocha.tmuxtheme")?;
    let reader = BufReader::new(file);

    let mut vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        vec.push(line);
    }

    vec[4] = format!("thm_bg='{}'", hex);

    let mut new = File::create("/home/hetzwga/.tmux/plugins/tmux/catppuccin-mocha.tmuxtheme")?;

    for line in &vec {
        writeln!(&mut new, "{}", line)?;
    }

    Command::new("tmux")
        .arg("source-file")
        .arg("/home/hetzwga/.tmux.conf")
        .spawn()
        .expect("source command failed to start");
    Ok(())
}
