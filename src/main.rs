mod encode;

use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use encode::compile;
use glob::glob;
use std::{
    fs::{write, File},
    io::Read,
    path::PathBuf,
    thread,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,
}

fn read_and_edit(path: PathBuf) -> Result<()> {
    let handle = thread::spawn(move || {
        println!("{} {:?}", "Transpiling file".yellow(), &path);
        let mut file = File::open(&path).expect("Could not open file!");
        let mut source = String::new();
        file.read_to_string(&mut source)
            .expect("Could not read file");
        let out = compile(source);
        write(&path, out).expect("Could not write file");
        println!("{} {:?}", "Transpiled file".green(), &path)
    });

    handle.join().unwrap();
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.path;

    if !path.exists() {
        bail!("Input does not exist: {}", path.to_string_lossy());
    }

    match path.metadata() {
        Ok(t) => {
            if t.is_dir() {
                println!("{}", "Recognised path as directory!".cyan());
                let temp = path.join("**/*.js");
                let glob_path = temp.as_os_str().to_str();
                match glob_path {
                    Some(h) => {
                        if !h.contains("__mocks__") && !h.contains("__tests__") {
                            println!("{} {:?}", "Searching glob pattern:".cyan(), h);
                            for entry in glob(h)? {
                                match entry {
                                    Ok(path) => {
                                        read_and_edit(path)?;
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    None => bail!("Could not parse glob path!"),
                }
            } else if t.is_file() {
                println!("{}", "Recognised path as file!".cyan());
                read_and_edit(path)?
            }
        }
        Err(_) => bail!("Could not access input metadata!"),
    }

    println!("{}", "Done".green());
    Ok(())
}
