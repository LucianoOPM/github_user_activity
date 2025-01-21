use std::{env, process};
mod structs;
use process::Command;
use structs::git_hub::Root;

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = match args.get(1) {
        Some(username) => username,
        None => {
            panic!("Please provide a username");
        }
    };

    let github_url = format!("https://api.github.com/users/{}/events", username);
    let output = Command::new("curl")
        .arg("-s")
        .arg(github_url)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let json: Root = serde_json::from_slice(&output.stdout).unwrap();
        for event in json {
            let event_type = event.type_field;
            let event_type = event_type.as_str();
            match event_type {
                "PushEvent" => {
                    let commit_len = event.payload.commits.len();
                    let repo_name = event.repo.name;
                    println!("- Pushed {} commits to {}", commit_len, repo_name);
                }
                "CreateEvent" => {
                    let create_type = event.payload.ref_type.expect("ref_type");
                    if create_type == "branch" {
                        let repo_name = event.repo.name;
                        println!("- Create a new branch {} on {}", create_type, repo_name);
                    } else if create_type == "repository" {
                        let repo_name = event.repo.name;
                        println!("- Create a new repository with name {}", repo_name);
                    } else if create_type == "tag" {
                        let repo_name = event.repo.name;
                        println!("- Created a new tag {} on {}", create_type, repo_name);
                    } else {
                        println!("Unknown Create Event");
                    }
                }
                "ReleaseEvent" => {
                    let release = event.payload.release.expect("release");
                    let repo_name = event.repo.name;
                    println!("- Released {} on {}", release.tag_name, repo_name);
                }
                "PullRequestEvent" => {
                    let pull_request = event.payload.pull_request.expect("pull_request");
                    let repo_name = event.repo.name;
                    println!(
                        "- Opened a pull request on {} with URL {}",
                        repo_name, pull_request.url
                    );
                }
                "DeleteEvent" => {
                    let delete_type = event.payload.ref_type.expect("ref_type");
                    if delete_type == "branch" {
                        let repo_name = event.repo.name;
                        println!("- Deleted a branch {} on {}", delete_type, repo_name);
                    } else if delete_type == "tag" {
                        let repo_name = event.repo.name;
                        println!("- Deleted a tag {} on {}", delete_type, repo_name);
                    } else {
                        println!("Unknown Delete Event");
                    }
                }
                _ => println!("Unknown Event: {}", event_type),
            };
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        panic!("Error: {}", error);
    }
}
