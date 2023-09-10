use clap::{Parser, Subcommand};
use popper_compiler::check_program;
use popper_compiler::get_ast;
use popper_compiler::popper_compile;

#[derive(Parser)]
/// Popper is a programming language
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// parse a file
    Parse {
        /// The name of the file to parse
        filename: String,
    },
    /// check a file
    Check {
        /// The name of the file to check
        filename: String,
    },
    /// compile a file
    Compile {
        /// The name of the file to compile
        filename: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Parse { filename } => {
            let ast = get_ast(&std::fs::read_to_string(filename).unwrap(), filename);
            match ast {
                Some(ast) => {
                    println!("{:#?}", ast);
                }
                None => {
                    println!("Error parsing file");
                }
            }
        }
        Commands::Check { filename } => {
            let ast = get_ast(&std::fs::read_to_string(filename).unwrap(), filename);
            match ast {
                Some(ast) => {
                    let res =
                        check_program(ast, &std::fs::read_to_string(filename).unwrap(), filename);
                    if res {
                        println!("No errors found");
                    }
                }
                None => {
                    println!("Error parsing file");
                }
            }
        }
        Commands::Compile { filename } => {
            let body = std::fs::read_to_string(filename).unwrap();
            let ir = popper_compile(body.as_str(), filename);
            println!("{}", ir);
        }
    }
}
