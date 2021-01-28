fn get_profile(profile: &str, command: &str) -> String {
    if profile == "default" {
        let shell_profile = include_str!("../../shell/default.txt");

        if command.len() > 0 {
            let new_command = format!("function {}", command);
            return shell_profile.replace("function nav", &new_command);
        }

        return shell_profile.to_string();
    }

    return "".to_string();
}

pub fn init(shell: String, command: Option<String>) {
    let mut profile: &str = "default";

    if shell == "bash" || shell == "zsh" {
        profile = "default"
    } else {
        println!(
            "echo -e \"\\033[0;31mError: Failed to load shell profile. Invalid profile provided.\""
        )
    }

    if let Some(cmd) = command {
        println!("{}", get_profile(profile, &cmd));
    } else {
        println!("{}", get_profile(profile, ""));
    }
}
