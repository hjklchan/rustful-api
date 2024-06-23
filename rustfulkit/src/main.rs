use clap::{Args, Parser, Subcommand};
use std::{fs, io::Write};

mod template;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
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
    static CRUD_FILES: [&'static str; 5] = ["create", "delete", "update", "get", "list"];

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
            CRUD_FILES.map(|operation| {
                // Opr as file name.
                let file_path = format!("{}/{}.rs", relative_path, operation);
                let template_str = template::make_handler_content(operation).unwrap();
                fs::File::create(file_path)
                    .unwrap()
                    .write(template_str.as_bytes())
                    .unwrap();
            });

            // Create mod.rs
            let mod_rs_path = format!("{}/mod.rs", relative_path);
            let template_str = template::make_mod_rs_content(module_name.to_string()).unwrap();
            fs::File::create(mod_rs_path)
                .unwrap()
                .write(template_str.as_bytes())
                .unwrap();
        }
        Err(why) => println!("! {:?}", why.kind()),
    }
}
