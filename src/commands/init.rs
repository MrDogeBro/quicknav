use anyhow::Result;

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

pub fn init(shell: String, command: Option<String>) -> Result<i32> {
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
