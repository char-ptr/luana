use std::fs::File;
use std::io::prelude::*;

use full_moon::parse;
use full_moon::visitors::VisitorMut;
use luana::vistAst;

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

fn main() {
    let proj = std::env::current_dir().unwrap().join("./lua_test");
    let mut main_file = File::open(proj.join("main.lua")).unwrap();
    let mut code = String::new();
    main_file.read_to_string(&mut code);
    test_speed!("generating ast",let ast = parse(&code));
    if let Ok(ast) = ast {
        let mut visit = vistAst::default();
        visit.set_project_dir(proj.clone());
        visit.set_file_name("main.lua");
        test_speed!("visiting ast",let ast = visit.visit_ast(ast));

        // println!("{}", full_moon::print(&ast));


        println!("done");
        File::create(proj.join("dist.lua")).unwrap().write_all(full_moon::print(&ast).trim().as_bytes());
    }
}
