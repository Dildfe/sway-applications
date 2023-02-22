use crate::utils::{read_applications, repo_root};
use std::{env::set_current_dir, process::Command};

pub(crate) fn run() {
    let root = repo_root();
    let apps = read_applications();

    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nTesting {}", app);

        let project = format!("{}/{}/project", root, app);
        set_current_dir(&project).expect(format!("Failed to change into: {}", project).as_str());

        let result = Command::new("cargo")
            .arg("test")
            .arg("--color")
            .arg("always")
            .arg("-q")
            .status();
        match result {
            Ok(status) => {
                if !status.success() {
                    errors.push(app.clone());
                }
            }
            Err(_) => errors.push(app.clone()),
        }
    }

    if 0 < errors.len() {
        println!("\nErrors found in");
        for app in errors.iter() {
            println!("    {}", app);
        }
    }
}
