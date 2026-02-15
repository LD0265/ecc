use clap::Parser;

/** Very basic mips compiler for a custom, c-like language
 Compiled files are stored in the same directory as the binary as <file.asm>
**/
#[derive(Parser, Debug)]
#[command(
    version = "0.1.5 by ElEmDee",
    about =
    "
Very basic mips compiler for a custom, c-like language
Compiled files are stored in the same directory as the binary as <file.asm>",
    long_about = None
)]
pub struct Args {
    /// Input file, must end in .ec extention
    #[arg(value_name = "file.ec")]
    pub input_file: String,

    /// Include comments in the generated assembly
    #[arg(short, long)]
    pub comment: bool,

    /// Print AST to stdout
    #[arg(short, long)]
    pub ast: bool,

    /// Print tokens to stdout
    #[arg(short, long)]
    pub tokens: bool,

    /// Name of the output file
    #[arg(short, default_value = "out.asm")]
    pub output: String,
}
