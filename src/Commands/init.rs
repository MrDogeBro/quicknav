pub fn init(shell: String) {
    let shell_default = include_str!("../../shell/default.txt");

    if shell == "bash" || shell == "zsh" {
        println!("{}", shell_default)
    } else {
        println!("echo -e \"\\033[0;31mError: Failed to load shell profile. Invalid profile provided.\"")
    }
}
