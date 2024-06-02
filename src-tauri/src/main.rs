// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::fs;
use varisat::{cnf, dimacs, solver::Solver, Lit};

const EXTENSION: &str = "cnf";

#[derive(Serialize, Deserialize, Debug)]
pub struct FormulaDetails {
    num_variables: usize,
    num_clauses: usize,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn select_formula() -> FormulaDetails {
    let foo = tauri_api::dialog::select(Some(EXTENSION), Some(".."));
    use tauri_api::dialog::Response::Okay;
    let choice = if let Ok(Okay(file_name)) = foo {
        file_name
    } else {
        return FormulaDetails {
            num_clauses: 0,
            num_variables: 0,
        };
    };

    dbg!(&choice);
    let mut solver = Solver::new();
    // let dimacs_cnf = b"1 2 3 0\n-1 -2 0\n-2 -3 0\n";

    let dimacs_cnf = fs::read(&choice).unwrap();
    let mut parser = dimacs::DimacsParser::new();
    parser.parse_chunk(dimacs_cnf.as_slice()).unwrap();
    let cnf_formula = parser.take_formula();

    solver.add_formula(&cnf_formula);
    let mut s2 = Solver::new();
    s2.add_formula(&cnf_formula);
    solver.assume(&[Lit::from_dimacs(-3), Lit::from_dimacs(5)]);

    FormulaDetails {
        num_clauses: cnf_formula.len(),
        num_variables: cnf_formula.var_count(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_formula])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
