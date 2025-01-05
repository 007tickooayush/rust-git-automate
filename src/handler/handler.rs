use std::process::{exit, Command};
use crate::utils::utils::get_generated_name;

pub fn update_commit_push() {
    let command_add =  Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("**\t\t|| ERROR: Failed to execute the \"git add .\" command.");

    if !command_add.status.success() {
        println!("**\t\t|| ERROR: Failed to execute the \"git add .\" command.");
        exit(1);
    }

    let generated_name = get_generated_name();
    let command_commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(generated_name)
        .output()
        .expect("**\t\t|| ERROR: Failed to execute the \"git commit -m\" command.");

    if !command_commit.status.success() {
        println!("**\t\t|| ERROR: Failed to execute the \"git commit -m\" command.");
        exit(1);
    }

    // CAN ONLY BE EXECUTED IF ORIGIN IS SET
    let command_push = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("**\t\t|| ERROR: Failed to execute the \"git push origin master\" command.");

    if !command_push.status.success() {
        println!("**\t\t|| ERROR: Failed to execute the \"git push origin master\" command.");
        exit(1);
    }

    println!("**\t\t|| SUCCESS: Successfully executed the \"git add .\", \"git commit -m\" and \"git push origin master\" commands.");

}