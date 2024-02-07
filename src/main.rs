use std::io::Write;
use popper_compiler::{compile_to_inkwell_llvm, compile_to_llvm, compile_to_mir, execute_llvm, pretty_mir};
use popper_compiler::get_ast;
use popper_compiler::check_program;
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author = "NightProg", version = "1.0.0", about = "The Popper CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// parsing a file
    Parse {
        #[arg(short, long)]
        json: bool,

        #[arg(value_hint = clap::ValueHint::DirPath)]
        file: std::path::PathBuf,

        #[arg(long, short, value_hint = clap::ValueHint::DirPath)]
        output: Option<std::path::PathBuf>
    },
    // check a file
    Check {
        #[arg(value_hint = clap::ValueHint::DirPath)]
        file: std::path::PathBuf
    },
    /// compile to the Popper MIR
    Mir {
        #[arg(value_hint = clap::ValueHint::DirPath)]
        file: std::path::PathBuf,

        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        output: Option<std::path::PathBuf>,
    },
    /// compile to LLVM Bytecode
    LLVM {
        #[arg(value_hint = clap::ValueHint::DirPath)]
        file: std::path::PathBuf,

        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        output: Option<std::path::PathBuf>,

        #[arg(short, long)]
        inkwell: bool

    },

    /// Run a popper file
    Run {
        #[arg(value_hint = clap::ValueHint::DirPath)]
        file: std::path::PathBuf,

        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        target: Option<std::path::PathBuf>,

        #[arg(short, long)]
        inkwell: bool,
    },

    Clean {
        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        target: Option<std::path::PathBuf>,

        #[arg(short = 'l', long)]
        only_libs: bool
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse {
            json, file, output
        } => {
            let string_file = file.to_str().expect("Unable to get a str");
            let content = std::fs::read_to_string(string_file).expect("File not found");
            let ast = get_ast(content.as_str(), string_file);
            if let Some(a) = ast {
                if json {
                    let s: String = serde_json::to_string_pretty(&a).unwrap();
                    if let Some(out) = output {
                        std::fs::File::open(out)
                            .expect("File Not Found")
                            .write(s.as_bytes())
                            .expect("Cannot write to file");
                    } else {
                        println!("{}", s);
                    };

                } else {
                    println!("{:#?}", a)
                }
            } else {
                eprintln!("Unable to parse file")
            }


        },
        Commands::Check {
            file
        } => {
            let string_file = file.to_str().expect("Unable to get a str");
            let content = std::fs::read_to_string(string_file).expect("File not found");
            let ast = get_ast(content.as_str(), string_file);
            if let Some(a) = ast {
                if check_program(a, content.as_str(), string_file) {
                    println!("Program is valid");
                } else {
                    println!("Program is invalid");
                }
            } else {
                eprintln!("Unable to parse file")
            }
        },
        Commands::Mir {
            file, output
        } => {
            let string_file = file.to_str().expect("Unable to get a str");
            let content = std::fs::read_to_string(string_file).expect("File not found");
            let ast = get_ast(content.as_str(), string_file);
            if let Some(a) = ast {
                if check_program(a.clone(), content.as_str(), string_file) {
                    let mir = compile_to_mir(a, string_file);
                    let res = pretty_mir(mir);
                    if let Some(out) = output {
                        std::fs::File::open(out)
                            .expect("File Not Found")
                            .write(res.as_bytes())
                            .expect("Cannot write to file");
                    } else {
                        println!("{}", res);
                    };
                } else {
                    println!("Program is invalid");
                }
            } else {
                eprintln!("Unable to parse file")
            }
        },
        Commands::LLVM {
            file, output, inkwell
        } => {
            let string_file = file.to_str().expect("Unable to get a str");
            let content = std::fs::read_to_string(string_file).expect("File not found");
            let ast = get_ast(content.as_str(), string_file);
            if let Some(a) = ast {
                if check_program(a.clone(), content.as_str(), string_file) {
                    let mir = compile_to_mir(a, string_file);
                    let res = if inkwell {
                        compile_to_inkwell_llvm(mir).0
                    } else {
                        compile_to_llvm(mir).0
                    };
                    if let Some(out) = output {
                        std::fs::File::open(out)
                            .expect("File Not Found")
                            .write(res.as_bytes())
                            .expect("Cannot write to file");
                    } else {
                        println!("{}", res);
                    };

                } else {
                    println!("Program is invalid");
                }
            } else {
                eprintln!("Unable to parse file")
            }
        },
        Commands::Run {
            file, target, inkwell
        } => {
            let string_file = file.to_str().expect("Unable to get a str");
            let content = std::fs::read_to_string(string_file).expect("File not found");
            let ast = get_ast(content.as_str(), string_file);
            if let Some(a) = ast {
                if check_program(a.clone(), content.as_str(), string_file) {
                    let mir = compile_to_mir(a, string_file);
                    let res = if inkwell {
                        compile_to_inkwell_llvm(mir)
                    } else {
                        let e = compile_to_llvm(mir);
                        (e.0, e.1.cdylib_used)
                    };
                    let target = target.unwrap_or(std::path::PathBuf::from("./target_popper"));
                    execute_llvm(res.0, string_file.to_string(), target.to_str().unwrap().to_string(), res.1);
                } else {
                    println!("Program is invalid");
                }
            } else {
                eprintln!("Unable to parse file")
            }
        },
        Commands::Clean { target , only_libs} => {
            let target = target.unwrap_or(std::path::PathBuf::from("./target_popper"));
            if only_libs {
                std::fs::remove_dir_all(target.join("libs")).expect("Unable to remove target directory");
            } else {
                std::fs::remove_dir_all(target).expect("Unable to remove target directory");
            }
        }
    }


}
