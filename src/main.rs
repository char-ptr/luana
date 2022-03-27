use std::fs::File;
use std::io::prelude::*;

use full_moon::parse;
use full_moon::visitors::VisitorMut;
use luana::VistAst;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};


macro_rules! test_speed {
    ($label:literal,$code:block) => {
        let start = std::time::Instant::now();
        $code;
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("⏱️{} took {}s",$label, duration.as_secs_f64());
    };
    ($label:literal,$code:stmt) => {
        let start = std::time::Instant::now();
        $code;
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("⏱️{} took {}s",$label, duration.as_secs_f64());
        // test_speed!($label,{$code});
    };
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: ProgramCommands,
}
#[derive(Subcommand,Debug)]

enum ProgramCommands {
    Init(InitCommand),
    Build(BuildCommand)
}
#[derive(clap::Args,Debug)]
struct InitCommand {
    // #[clap(short,long)]
    name: String,
    #[clap(parse(from_os_str),default_value=".")]
    path: std::path::PathBuf,
}
#[derive(clap::Args,Debug)]
struct BuildCommand {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
// fn main() {
//     let proj = std::env::current_dir().unwrap().join("./lua_test");
//     let mut main_file = File::open(proj.join("main.lua")).unwrap();
//     let mut code = String::new();
//     main_file.read_to_string(&mut code);
//     test_speed!("generating ast",let ast = parse(&code));
//     if let Ok(ast) = ast {
//         let mut visit = VistAst::default();
//         visit.set_project_dir(proj.clone());
//         visit.set_file_name("main.lua");
//         test_speed!("visiting ast",let ast = visit.visit_ast(ast));
//         let mut minifier = luana::minify::MinifyVisiter::default();
//         test_speed!("minifing",let ast = minifier.visit_ast(ast));

//         // println!("{}", full_moon::print(&ast));


//         println!("done");
//         File::create(proj.join("dist.lua")).unwrap().write_all(full_moon::print(&ast).trim().as_bytes());
//     }
// }
#[derive(Serialize, Deserialize, Debug)]
struct ProjectSettings {
    entry_point: String,
    minification: bool,
}

fn main() {

    let args = CliArgs::parse();

    match args.command {
        ProgramCommands::Init(init_args) => {
            let proj = init_args.path.join(&init_args.name);
            std::fs::create_dir_all(proj.clone()).unwrap();
            let settings_file = File::create(proj.join(".luana.json")).unwrap();
            let entry = File::create(proj.join("main.lua")).unwrap();
            let settings = ProjectSettings {
                entry_point:"main.lua".into(),
                minification: true,
            };
            serde_json::to_writer_pretty(settings_file, &settings).expect("unable to write project settings 😢");

            println!("🎉  Project {} created at {}", init_args.name,proj.canonicalize().unwrap().to_string_lossy());
        }
        ProgramCommands::Build(build_args) => {
            let proj = build_args.path;
            if let Ok(settings_file) = File::open(proj.join(".luana.json")) {
                let settings:ProjectSettings = serde_json::from_reader(settings_file).expect("Invaid json inside .luana.json");

                let mut entry = File::open(proj.join(&settings.entry_point)).expect("Unable to find the entry file provided in .luana.json");

                let mut code = String::new();
                entry.read_to_string(&mut code);
                test_speed!("generating ast",let ast = parse(&code));
                if let Ok(ast) = ast {
                    let mut visit = VistAst::default();
                    visit.set_project_dir(proj.clone());
                    visit.set_file_name(&settings.entry_point);
                    test_speed!("visiting ast",let mut ast = visit.visit_ast(ast));
                    if settings.minification {
                        let mut minifier = luana::minify::MinifyVisiter::default();
                        test_speed!("minifing",ast = minifier.visit_ast(ast));
                    }
                    let output_path = proj.join("dist.lua");
                    println!("Done! writing to {}",&output_path.to_string_lossy());
                    File::create(output_path).unwrap().write_all(full_moon::print(&ast).trim().as_bytes());
                } else {
                    println!("was unable to generate an ast from the file, are you sure it's a valid lua file? 🤔");
                }

            } else {
                println!("The path you provided is not a luana project. you can create one with luana init <name>");
            }
        }
    }
}