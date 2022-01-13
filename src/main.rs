
use clap::StructOpt;
use cli::DNA;

mod cli;
mod toml;

fn main() {
    
    let dna = DNA::parse();
    dna.execute_sub_command();
    
}
