extern crate tera;


mod prelude;

use std::{fs::{self, File}, io::Write};
use prelude::*;

use clap::{Parser, Subcommand};
use tera::{Tera, Context};

#[derive(Debug, Parser)]
#[command(name = "generate")]
#[command(about = "generate new files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Debug, Subcommand)]
enum Commands {
    Component {
       name: String,
       dest: String,
    },
    Page {
        name: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let Ok(tera) = Tera::new("templates/**/*") else {
        panic!("templates not found parsing error");
    };

    let mut context = Context::new();

    match args.command {
        Commands::Component { name, dest } => {
            context.insert("name", &name);
            let result = match tera.render("component/index.tsx", &context) {
                Ok(it) => it,
                Err(_err) => return Err(Error::Static("cannot parse templates")),
            };
            let folder_path = f!("{}/{}", dest, name.to_lowercase());
            fs::create_dir_all(folder_path)?;
            let path = f!("{}/{}/index.tsx", dest, name.to_lowercase());
            let mut file = File::create(path).expect("Unable to create file");
            file.write_all(result.as_bytes()).expect("Unable to write data");
        },
        Commands::Page { name } => {
            context.insert("name", &name);
        },
    };
    Ok(())
}
