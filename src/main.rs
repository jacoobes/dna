use clap::StructOpt;
use cli::DNA;

mod cli;

fn main() {
    let dna = DNA::parse();
    dna.execute_sub_command();
    
}
