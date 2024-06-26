use clap::{Args, Parser, Subcommand};
use rustfulkit::template::{handler_template, mod_template};
use std::{fs, io::Write};

// TODO: Do some optimisation
// 1. Error handing
// 2. Organize path-related variables

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Create(CreateArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct CreateArgs {
    #[command(subcommand)]
    command: Option<CreateCommands>,
}

#[derive(Debug, Subcommand)]
enum CreateCommands {
    Module(ModuleArgs),
}

#[derive(Debug, Args)]
struct ModuleArgs {
    #[arg(short, long)]
    name: Option<String>,
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create(args) => match args.command.unwrap() {
            CreateCommands::Module(args) => {
                create_module(args.path, args.name);
            }
        },
    }
}

fn create_module(path: Option<String>, name: Option<String>) -> () {
    static OPERATIONS: [&'static str; 5] = ["create", "delete", "update", "get", "list"];

    let module_name = match &name {
        Some(value) => value,
        None => "example",
    };
    let path = match &path {
        Some(value) => value,
        None => "./src/http",
    };

    // Create a new folder called currently module name
    let relative_path = &format!("{}/{}", path, module_name);
    match fs::create_dir(relative_path) {
        Ok(()) => {
            // Create CRUD rs files.
            OPERATIONS.map(|operation| {
                // Opr as file name.
                let file_path = format!("{}/{}.rs", relative_path, operation);
                let template_str = handler_template::make(operation).unwrap();
                fs::File::create(file_path)
                    .unwrap()
                    .write(template_str.as_bytes())
                    .unwrap();
            });

            // Create mod.rs
            let mod_rs_path = format!("{}/mod.rs", relative_path);
            let template_str = mod_template::make(module_name.to_string()).unwrap();
            fs::File::create(mod_rs_path)
                .unwrap()
                .write(template_str.as_bytes())
                .unwrap();
        },
        Err(why) => println!("! {:?}", why.kind()),
    }
}
