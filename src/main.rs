extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;

mod commands;
mod config;

use gag::BufferRedirect;
use std::io::Read;
use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Quicknav {
    /// Gets the location of a provided shortcut
    Get {
        /// The location to find, known as a call in the
        /// config file
        location: String,
    },
    /// Lists the registered shortcuts
    List {
        /// The shortcut to search for
        shortcut: Option<String>,
    },
    /// Adds a new shortcut
    Add {
        /// The shortcut itself (call)
        shortcut: String,
        /// The shortcut location
        location: String,
        /// The shortcut name
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
        /// The shortcut description
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    /// Removes a shortcut
    Remove {
        /// The shortcut to remove (by call)
        shortcut: String,
    },
    /// Initalizes the shell profile
    Init {
        /// The shell profile to use
        shell: String,
        /// Optional way to change the invoke command
        #[structopt(short = "c", long = "command")]
        command: Option<String>,
    },
}

fn main() {
    // handle command dispatching
    match Quicknav::from_args() {
        Quicknav::Get { location } => {
            commands::get(location);
        }
        Quicknav::List { shortcut } => {
            commands::list(shortcut);
        }
        Quicknav::Add {
            shortcut,
            location,
            name,
            description,
        } => {
            commands::add(shortcut, location, name, description);
        }
        Quicknav::Remove { shortcut } => commands::remove(shortcut),
        Quicknav::Init { shell, command } => {
            let supported_shells = vec!["bash", "zsh", "fish"];
            if supported_shells.iter().any(|&s| s == shell) {
                commands::init(shell.to_owned(), command);
                gen_completions(shell);
            } else {
                println!(
                    "echo -e \"\\033[0;31mError: Failed to load shell profile. Invalid or unsupported shell provided.\""
                );
            }
        }
    }
}

fn gen_completions(shell: String) {
    let mut shell_profile = Shell::Bash;

    if shell == "bash" {
        shell_profile = Shell::Bash;
    } else if shell == "zsh" {
        shell_profile = Shell::Bash;

        let mut stdout_buf = BufferRedirect::stdout().unwrap();
        Quicknav::clap().gen_completions_to("quicknav", shell_profile, &mut std::io::stdout());

        let mut completions = String::new();
        stdout_buf.read_to_string(&mut completions).unwrap();
        drop(stdout_buf);

        println!("{}", completions);

        return;
    } else if shell == "fish" {
        shell_profile = Shell::Fish;
    }

    Quicknav::clap().gen_completions_to("quicknav", shell_profile, &mut std::io::stdout());
}
