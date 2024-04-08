use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

fn is_git_initialized(path: &Path) -> bool {
    path.join(".git").exists()
}

fn has_uncommitted_changes(path: &Path) -> bool {
    let output = match Command::new("git")
        .args(&["status", "--porcelain"])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(_) => return false,
    };

    if !output.status.success() {
        return false;
    }

    let stdout_str = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return false,
    };

    !stdout_str.trim().is_empty()
}

fn add_uncommitted_changes(path: &Path) -> bool {
    println!("Adding changes for path: {}", path.display());

    let output = match Command::new("git")
        .args(&["add", "."])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // Capture stderr for error message
        .output()
    {
        Ok(output) => {
            // println!("Output : {:?}", output.stdout);
            output
        },
        Err(e) => {
            println!("Failed to execute 'git add' command: {}", e);
            return false
        }
    };

    if !output.status.success() {
        if let Some(error_message) = String::from_utf8(output.stderr).ok() {
            println!("Error message from 'git add' command: {}", error_message);
        } else {
            println!("Failed to execute 'git add' command");
        }
        return false;
    }

    let stdout_str = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to parse stdout: {}", e);
            return false;
        }
    };

    stdout_str.trim().is_empty()
}

fn commit_uncommitted_changes(path: &Path, message: &str) -> bool {
    let output = match Command::new("git")
        .args(&["commit", "-m", message])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => {
            // println!("Output : {:?}", output.stdout);
            output
        },
        Err(e) => {
            println!("Failed to execute 'git add' command: {}", e);
            return false
        }
    };

    if !output.status.success() {
        return false;
    }

    let stdout_str = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return false,
    };

    !stdout_str.trim().is_empty()
}

fn push_uncommitted_changes(path: &Path,branch_name: &str) -> bool {
    let mut command = Command::new("git");
    command.args(&["push","--set-upstream","origin",branch_name]).current_dir(path).stdout(Stdio::piped()).stderr(Stdio::piped());

    if let Ok(child) = command.spawn() {
        let output = child.wait_with_output().expect("Failed to wait for git push process");

        if output.status.success() {
            // let stdout_str = String::from_utf8(output.stdout).unwrap();
            // println!("Output : {}", stdout_str);
            return true;
        } else {
            let stderr_str = String::from_utf8(output.stderr).unwrap();
            println!("Error message from 'git push' command: {}", stderr_str);
        }
    } else {
        println!("Failed to execute 'git push' command");
    }

    false
}






fn reset_uncommitted_changes(path: &Path) -> bool {
    let mut command = Command::new("git");
    command.args(&["reset","--soft","HEAD^"]).current_dir(path).stdout(Stdio::piped()).stderr(Stdio::piped());

    if let Ok(child) = command.spawn() {
        let output = child.wait_with_output().expect("Failed to wait for git reset process");

        if output.status.success() {
            // let stdout_str = String::from_utf8(output.stdout).unwrap();
            // println!("Output : {}", stdout_str);
            return true;
        } else {
            let stderr_str = String::from_utf8(output.stderr).unwrap();
            println!("Error message from 'git reset' command: {}", stderr_str);
        }
    } else {
        println!("Failed to execute 'git reset' command");
    }

    false
}






fn branch_uncommitted_changes(path: &Path,message: &str) -> bool {
    let mut command = Command::new("git");
    command.args(&["checkout","-b",message]).current_dir(path).stdout(Stdio::piped()).stderr(Stdio::piped());

    if let Ok(child) = command.spawn() {
        let output = child.wait_with_output().expect("Failed to wait for git branch process");

        if output.status.success() {
            // let stdout_str = String::from_utf8(output.stdout).unwrap();
            // println!("Output : {}", stdout_str);
            return true;
        } else {
            let stderr_str = String::from_utf8(output.stderr).unwrap();
            println!("Error message from 'git checkout' command: {}", stderr_str);
        }
    } else {
        println!("Failed to execute 'git checkout' command");
    }

    false
}






fn master_branch_uncommitted_changes(path: &Path) -> bool {
    let mut command = Command::new("git");
    command.args(&["checkout","master"]).current_dir(path).stdout(Stdio::piped()).stderr(Stdio::piped());

    if let Ok(child) = command.spawn() {
        let output = child.wait_with_output().expect("Failed to wait for git branch master process");

        if output.status.success() {
            // let stdout_str = String::from_utf8(output.stdout).unwrap();
            // println!("Output : {}", stdout_str);
            return true;
        } else {
            let stderr_str = String::from_utf8(output.stderr).unwrap();
            println!("Error message from 'git checkout master' command: {}", stderr_str);
        }
    } else {
        println!("Failed to execute 'git checkout master' command");
    }

    false
}



fn remove_branch_uncommitted_changes(path: &Path,message: &str) -> bool {
    let mut command = Command::new("git");
    command.args(&["branch","-D",message]).current_dir(path).stdout(Stdio::piped()).stderr(Stdio::piped());

    if let Ok(child) = command.spawn() {
        let output = child.wait_with_output().expect("Failed to wait for git branch process");

        if output.status.success() {
            // let stdout_str = String::from_utf8(output.stdout).unwrap();
            // println!("Output : {}", stdout_str);
            return true;
        } else {
            let stderr_str = String::from_utf8(output.stderr).unwrap();
            println!("Error message from 'git checkout' command: {}", stderr_str);
        }
    } else {
        println!("Failed to execute 'git checkout' command");
    }

    false
}

fn traverse_directory(path: &Path, skip_arr: &[&str], found_paths: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let dir_name = entry_path.file_name().unwrap().to_str().unwrap();
                    if !skip_arr.contains(&dir_name) {
                        if is_git_initialized(&entry_path) && has_uncommitted_changes(&entry_path) {
                            found_paths.push(entry_path.clone());
                        }
                        traverse_directory(&entry_path, skip_arr, found_paths);
                    }
                }
            }
        }
    }
}





fn format_seconds_since_epoch(seconds: u64) -> String {
    let (secs, mins, hrs, days) = (
        seconds % 60,
        (seconds / 60) % 60,
        (seconds / 3600) % 24,
        seconds / 86400,
    );

    format!(
        "auto commit at {} days, {} hours, {} minutes, {} seconds since the Unix epoch",
        days, hrs, mins, secs
    )
}





fn main() {
    let root_path = Path::new("C:\\Users\\satvi\\Documents\\Repos");
    let skip_arr = ["node_modules", "env", ".idea", "temp"];
    let mut found_paths: Vec<PathBuf> = Vec::new();
    traverse_directory(&root_path, &skip_arr, &mut found_paths);

    if !found_paths.is_empty() {
        for path in &found_paths {
            println!("Git initialized directory with uncommitted changes: {:?}", path);
        }

        println!("Do you wish to commit changes? (Y/N)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_ascii_lowercase() == "y" {
            for path in &found_paths {
                if add_uncommitted_changes(path) {
                    let now = SystemTime::now();
                    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
                    let seconds = duration_since_epoch.as_secs();

                    let formatted_string = format_seconds_since_epoch(seconds);
                    let branch_name = format!("temp_branch_{}", seconds);

                    let mut success = false;

                    if branch_uncommitted_changes(path, &branch_name){
                        println!("Switched branch successfully.");
                        if commit_uncommitted_changes(path, &formatted_string) {
                            println!("Changes committed successfully.");
                            if push_uncommitted_changes(path,&branch_name) {
                                println!("Changes pushed successfully.");
                                success = true;
                            }else{
                                println!("Changes not pushed.");
                            }
                        }
                    }

                    if !success{
                        println!("removing branch");
                        if reset_uncommitted_changes(path){
                            if master_branch_uncommitted_changes(path){
                                if remove_branch_uncommitted_changes(path,&branch_name){
                                    println!("removed branch");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

