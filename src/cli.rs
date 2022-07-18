use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Rusty Virtual Machine",
    about = "Virtual Machine simulating a fictional computer called the LC-3"
)]
pub struct CliArgs {
    /// The path to the obj file to read
    pub input_file_path: String,
    /// Verbosity level
    /// -v for debug,
    /// -vv for trace
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbose: u8,
}
