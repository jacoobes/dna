use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
pub struct DNA {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[clap(about, version, author)]
pub enum Command {
    New {
        #[clap(short, long)]
        name: String,
    },
}

impl DNA {
    pub fn execute_sub_command(&self) {
        match &self.command {
            Command::New { name } => {
                let cur_dir = std::env::current_dir().expect(
                    "Either have no permissions to open directory or directory does not exist",
                );
                let config_file = "main.toml";

                let toml = format!("# Your base module name\npackage = \"{}\"\n# This is the entry point to Nums project\nmain = \"src/main.nums\"", &name);
                let toml_path = cur_dir.join(config_file);

                let mut file = create_open_options(&toml_path, "Could not open file : ");
                write(&mut file, toml.as_bytes());

                fs::create_dir(cur_dir.join("src")).expect("Could not create main directory `src`");
                let mut main = create_open_options(
                    &cur_dir.join("src/main.nums"),
                    "Could not create main.nums in src dir!",
                );
                write(&mut main, b"package main;\n\tfun main { :> main function <: }" );
            }
        }
    }
}
fn create_open_options(path: &PathBuf, error_msg: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect(error_msg)
}

fn write(file: &mut File, buf: &[u8]) {  file.write_all(buf).expect("could not write to file") }
