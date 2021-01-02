extern crate reqwest;
extern crate text_io;
extern crate git2;

use git2::Repository;
use std::{fs, io};
use text_io::read;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let subject_folders = list_subject_folders();
    for folder in subject_folders.clone() {
        println!("{}", folder)
    }
    println!("Select Subject:");
    let mut valid_subject_choice: bool = false;
    let mut subject_choice: String = String::from("");
    while !valid_subject_choice {
        subject_choice = read!();
        if subject_folders.contains(&subject_choice) {
            valid_subject_choice = true;
        } else {
            println!("{}", format!("{} doesn't exist. Create now? (y/n)", subject_choice));
            let new_sub: String = read!();
            if new_sub.to_lowercase().eq("y") {
                println!("Creating new subject {}", subject_choice.clone());
                create_new_repo(PathBuf::from(subject_choice.clone()));
                valid_subject_choice = true;
            }
        }
    }
    println!("Enter Document name:");
    let document_choice: String = read!();
    println!("Creating document {:?}", PathBuf::from(".").join(subject_choice.clone()).join(document_choice.clone()));
    create_new_document(PathBuf::from(".").join(subject_choice.clone()).join(document_choice.clone()));
    println!("Done!\nPress enter to continue...");
    let _: String = read!("{}\n");
    Ok(())
}

fn create_new_repo(folder: PathBuf) {
    println!("Creating new repo");
    Repository::init(folder.clone()).expect("Unable to init new repository");
    write_files(folder)
}

fn create_new_document(folder: PathBuf) {
    println!("Downloading template");
    fs::create_dir_all(folder.clone().join("src")).expect("Unable to create document folder");
    fs::write(folder.clone().join("src").join("main.tex"), get("https://gist.githubusercontent.com/NastyGamer/722a29264a7d3bad7b0157097b9ec1b2/raw/df1753fcc8710ff5e40427345631cdc2f77e0df4/LaTeX-Template.tex".parse().unwrap())).expect("Unable to write template");
    fs::write(folder.clone().join("src").join("solarized.sty"), get("https://raw.githubusercontent.com/jez/latex-solarized/master/solarized.sty".parse().unwrap())).expect("Unabel to write solarized style");
    fs::write(folder.clone().join("src").join("solarized-dark.sty"), get("https://raw.githubusercontent.com/jez/latex-solarized/master/solarized-dark.sty".parse().unwrap())).expect("Unable to write solarized dark style");
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