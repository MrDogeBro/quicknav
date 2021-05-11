use anyhow::Result;
use gag::BufferRedirect;
use quicknav::Quicknav;
use std::io::Read;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::quicknav;

fn get_profile(profile: &str, command: &str) -> Result<String> {
    if profile == "default" {
        let shell_profile = include_str!("../../shell/default.txt");

        if command.len() > 0 {
            let new_command = format!("function {}", command);
            return Ok(shell_profile.replace("function nav", &new_command));
        }

        return Ok(shell_profile.to_string());
    } else if profile == "fish" {
        let shell_profile = include_str!("../../shell/fish.txt");

        if command.len() > 0 {
            let new_command = format!("function {}", command);
            return Ok(shell_profile.replace("function nav", &new_command));
        }

        return Ok(shell_profile.to_string());
    }

    Ok("".to_string())
}

fn gen_completions(shell: String) -> Result<i32> {
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

        println!(
            "autoload bashcompinit\nbashcompinit\n\n{}",
            completions.replace(
                "complete -F _quicknav -o bashdefault -o default quicknav",
                "$(autoload | grep -q bashcompinit) && \
                 complete -F _quicknav -o bashdefault -o default quicknav"
            )
        );

        return Ok(0);
    } else if shell == "fish" {
        shell_profile = Shell::Fish;
    }

    Quicknav::clap().gen_completions_to("quicknav", shell_profile, &mut std::io::stdout());

    Ok(0)
}

pub fn init(shell: String, command: Option<String>) -> Result<i32> {
    let supported_shells = vec!["bash", "zsh", "fish"];

    if supported_shells.iter().any(|&s| s == shell) {
        gen_completions(shell.to_owned())?;
    } else {
        println!("echo -e \"\\033[0;31mError:\\033[0m Failed to load shell profile. Invalid or unsupported shell provided.\"");
        return Ok(1);
    }

    let mut profile: &str = "default";

    if shell == "bash" || shell == "zsh" {
        profile = "default"
    } else if shell == "fish" {
        profile = "fish"
    }

    if let Some(cmd) = command {
        println!("{}", get_profile(profile, &cmd)?);
    } else {
        println!("{}", get_profile(profile, "")?);
    }

    Ok(0)
}
