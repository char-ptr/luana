use std::fs::File;
use std::io::prelude::*;

use full_moon::parse;
use full_moon::visitors::VisitorMut;
use luana::vistAst;

fn main() {
    let proj = std::env::current_dir().unwrap().join("./lua_test");
    let mut main_file = File::open(proj.join("main.lua")).unwrap();
    let mut code = String::new();
    main_file.read_to_string(&mut code);
    let ast = parse(&code);
    if let Ok(ast) = ast {
        let mut visit = vistAst::default();
        visit.set_project_dir(proj.clone());
        visit.set_file_name("main.lua");
        let ast = visit.visit_ast(ast);

        // println!("{}", full_moon::print(&ast));


        println!("done");
        File::create(proj.join("dist.lua")).unwrap().write_all(full_moon::print(&ast).trim().as_bytes());
    }
}
