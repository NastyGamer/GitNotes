extern crate reqwest;
extern crate text_io;

use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

use text_io::read;

const DEBUG: bool = true;

fn main() -> io::Result<()> {
    match Command::new("git").arg("--version").output() {
        Ok(v) => { println!("{}", from_utf8(&*v.stdout).unwrap()) }
        Err(_) => { panic!("Git not found") }
    }
    let subject_folders = list_subject_folders();
    if !subject_folders.contains(&String::from(".git")) {
        println!("Preparing workspace...");
        Command::new("git")
            .arg("init")
            .current_dir(".")
            .output()
            .expect("Unable to prepare workspace");
    }
    println!("Available subjects:");
    for folder in &subject_folders {
        println!("\t{}", folder);
    }
    let mut valid_subject_choice: bool = false;
    let mut subject_choice: String = String::from("");
    while !valid_subject_choice {
        println!("Select subject:");
        subject_choice = read!();
        if subject_folders.contains(&subject_choice) {
            valid_subject_choice = true;
        } else {
            println!("{} doesn't exist. Create it? (y/n)", &subject_choice);
            let create_new_subject: String = read!();
            if create_new_subject.to_lowercase().eq("y") {
                println!("Creating new subject {}...", &subject_choice);
                create_new_repo(PathBuf::from(&subject_choice));
                valid_subject_choice = true;
            }
        }
    }
    println!("Enter Document name:");
    let document_choice: String = read!();
    println!("Creating document {:?}...", PathBuf::from(".").join(&subject_choice).join(&document_choice));
    create_new_document(PathBuf::from(".").join(&subject_choice).join(&document_choice));
    println!("Done!\nPress enter to continue...");
    let _: String = read!("{}\n");
    Ok(())
}

fn create_new_repo(folder: PathBuf) {
    println!("Creating new repo");
    fs::create_dir(&folder).expect("Unable to create subject folder");
    if DEBUG { println!("Running git init in {:?}", &folder); }
    Command::new("git")
        .arg("init")
        .current_dir(&folder)
        .output()
        .expect("Unable to init new repo");
    write_files(folder.clone());
    if DEBUG { println!("Running git add * in {:?}", &folder); }
    Command::new("git")
        .args(&["add", "*"])
        .current_dir(&folder)
        .output()
        .expect("Unable to add files for commit");
    if DEBUG { println!("Running git commit -m \"Initial commit\" in {:?}", &folder); }
    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&folder)
        .output()
        .expect("Unable to commit changes");
    if DEBUG { println!("Running git submodule add ./{} in .", &folder.file_name().unwrap().to_str().unwrap()); }
    Command::new("git")
        .args(&["submodule", "add", &(String::from("./") + &folder.file_name().unwrap().to_str().unwrap())])
        .current_dir(".")
        .output()
        .expect("Unable to add submodule");
}

fn create_new_document(folder: PathBuf) {
    println!("Downloading template");
    fs::create_dir_all(folder.join("src")).expect("Unable to create document folder");
    fs::write(folder.join("src").join("main.tex"), get("https://gist.githubusercontent.com/NastyGamer/722a29264a7d3bad7b0157097b9ec1b2/raw/df1753fcc8710ff5e40427345631cdc2f77e0df4/LaTeX-Template.tex".parse().unwrap())).expect("Unable to write template");
    fs::write(folder.join("src").join("solarized.sty"), get("https://raw.githubusercontent.com/jez/latex-solarized/master/solarized.sty".parse().unwrap())).expect("Unabel to write solarized style");
    fs::write(folder.join("src").join("solarized-dark.sty"), get("https://raw.githubusercontent.com/jez/latex-solarized/master/solarized-dark.sty".parse().unwrap())).expect("Unable to write solarized dark style");
}

fn list_subject_folders() -> Vec<String> {
    fs::read_dir(".").unwrap()
        .filter(|res| res.as_ref().unwrap().path().is_dir())
        .map(|res| String::from(res.unwrap().file_name().to_str().unwrap()))
        .collect::<Vec<String>>()
}

fn write_files(folder: PathBuf) {
    fs::write(folder.join(".gitignore"), get("https://www.toptal.com/developers/gitignore/api/latex,jetbrains,vscode,windows,macos,linux".parse().unwrap())).expect("Unable to write .gitignore");
}

fn get(url: String) -> String {
    return reqwest::blocking::get(&url).unwrap().text().unwrap();
}