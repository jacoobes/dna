use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{PathBuf, Path}
};
use nums::{
    compiler::{nums_compiler::Compiler, source::Source}
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

    Build
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
            Command::Build => {
                let working_dir = std::env::current_dir()
                .expect("Either have no permissions to open directory or directory does not exist")
                .join("main.toml");
                let toml_str = fs::read_to_string(working_dir).expect("Cannot read main.toml file!");
                let main_data = crate::toml::MainToml::parse(&toml_str).expect("Could not read toml file!");
                
             if main_data
                .package()
                .chars()
                .all(|x| !char::is_ascii_alphabetic(&x) && char::is_whitespace(x)) {
                    panic!("{}, your package name has to be alphabetic and contain no whitespace!", main_data.package())
                }
                let entry_pt = std::env::current_dir().expect("The current dir does not exist").join(main_data.main());
                let file = read_file(&entry_pt);
                let compiler  = Compiler::new(Source::new(file, main_data.main()));
                compiler.compile(main_data.package())
                
            },
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
fn read_file<P : AsRef<Path>>(path: P) -> String { fs::read_to_string(path).expect("Unable to read file") }