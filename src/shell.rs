use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::Command,
};

use dirs::home_dir;

pub enum Shell {
    Fish,
    Bash,
    Zsh,
    Unknown,
}

impl Shell {
    pub fn get_history_path(&self) -> Result<PathBuf, io::Error> {
        match self {
            Shell::Fish => {
                let home = home_dir().unwrap_or_default();
                Ok(home.join(".local/share/fish/fish_history"))
            }
            Shell::Bash => {
                let home = home_dir().unwrap_or_default();
                Ok(home.join(".bash_history"))
            }
            Shell::Zsh => {
                let home = home_dir().unwrap_or_default();
                Ok(home.join(".zsh_history"))
            }
            Shell::Unknown => Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown shell, unable to determine history file",
            )),
        }
    }

    fn read_history_file(&self) -> io::Result<Vec<String>> {
        let path = self.get_history_path()?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>()?;
        Ok(lines)
    }

    pub fn get_commands(&self) -> Result<Vec<String>, io::Error> {
        let lines = self.read_history_file()?;
        match self {
            Shell::Fish => Ok(lines
                .into_iter()
                .filter(|line| line.starts_with("- cmd:"))
                .map(|line| line.trim_start_matches("- cmd:").trim().to_string())
                .collect()),
            Shell::Bash => Ok(lines),
            Shell::Zsh => Ok(lines
                .into_iter()
                .map(|line| {
                    if let Some(cmd) = line.split(';').nth(1) {
                        cmd.to_string()
                    } else {
                        line
                    }
                })
                .collect()),
            Shell::Unknown => Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown shell, unable to determine history file",
            )),
        }
    }
}

pub fn detect_shell() -> Shell {
    if let Ok(output) = Command::new("ps")
        .arg("-p")
        .arg(format!("{}", unsafe { libc::getppid() }))
        .output()
    {
        let output = String::from_utf8_lossy(&output.stdout);
        if output.contains("fish") {
            return Shell::Fish;
        } else if output.contains("bash") {
            return Shell::Bash;
        } else if output.contains("zsh") {
            return Shell::Zsh;
        }
    }
    Shell::Unknown
}
