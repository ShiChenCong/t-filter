use std::{env, fs, process::Command};

use home::home_dir;

//  ~/.bash_history bash
//  ~/.zsh_history  zsh

// shell类型
enum ShellTypeEnum {
    FISH,
    ZSH,
}

fn get_shell_name() -> Result<ShellTypeEnum, String> {
    if let Ok(shell) = env::var("SHELL") {
        if shell.contains("fish") {
            Ok(ShellTypeEnum::FISH)
        } else if shell.contains("zsh") {
            Ok(ShellTypeEnum::ZSH)
        } else {
            Err(String::from("获取失败"))
        }
    } else {
        Err(String::from("获取失败"))
    }
}

pub fn get_command_history() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match get_shell_name() {
        Ok(ShellTypeEnum::FISH) => {
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
                Err(String::from(stderr).into())
            }
        }
        Ok(ShellTypeEnum::ZSH) => {
            // let a = Path::new("./zsh_history");
            match home_dir() {
                Some(path) => {
                    // let total_path: PathBuf = [path.display(), "zsh_history"].iter().collect();
                    let mut a = path.clone();
                    a.push(".zsh_history");
                    let file_reader = fs::read(a)?;
                    let result = String::from_utf8_lossy(&file_reader);
                    let content: Vec<String> = result
                        .split("\n")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&s| {
                            let a = s.split(";").last().unwrap();
                            a.to_string()
                        })
                        .rev()
                        .collect();
                    Ok(content)
                }
                None => Err(String::from("2").into()),
            }
            // Ok(Vec::from([String::from("1")]))
        }
        Err(e) => Err(String::from(e).into()),
    }
}
