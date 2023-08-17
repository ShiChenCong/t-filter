use std::process::Command;

//  ~/.bash_history bash
//  ~/.zsh_history  zsh
pub fn get_command_history() -> Result<Vec<String>, String> {
    let output = Command::new("fish")
        .arg("-c")
        .arg("history")
        .output()
        .unwrap();

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let result = stdout
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&s| s.to_string())
            .collect();
        Ok(result)
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        Err(stderr)
    }
}
