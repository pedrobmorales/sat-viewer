// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use varisat::{dimacs, solver::Solver, Lit};

const EXTENSION: &str = "cnf";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn select_formula(name: &str) -> String {
    let foo = tauri_api::dialog::select(Some(EXTENSION), Some(".."));
    use tauri_api::dialog::Response::Okay;
    let choice = if let Ok(Okay(file_name)) = foo {
        file_name
    } else {
        return "File not selected or not found".to_string();
    };

    dbg!(&choice);
    let mut solver = Solver::new();
    // let dimacs_cnf = b"1 2 3 0\n-1 -2 0\n-2 -3 0\n";

    let dimacs_cnf = fs::read(choice).unwrap();
    let mut parser = dimacs::DimacsParser::new();
    parser.parse_chunk(dimacs_cnf.as_slice()).unwrap();
    let cnf_formula = parser.take_formula();

    for lits in cnf_formula.iter() {
        dbg!(&lits);
    }
    // dbg!(cnf_formula);
    solver.add_formula(&cnf_formula);
    let mut s2 = Solver::new();
    s2.add_formula(&cnf_formula);
    solver.assume(&[Lit::from_dimacs(-3), Lit::from_dimacs(5)]);
    let solution = solver.solve().unwrap();
    let model = solver.model().unwrap();

    format!("{:?}", model)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_formula])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
