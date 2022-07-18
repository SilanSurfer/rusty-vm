use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;

mod cli;
mod machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::CliArgs::from_args();
    let file = File::open(args.input_file_path)?;
    let reader = BufReader::new(file);

    let mut vm = machine::Machine::new();
    vm.save_program_to_memory(reader);
    vm.run();
    Ok(())
}
