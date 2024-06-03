// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::fs;
use tauri_api::dialog::select;
use varisat::{cnf, dimacs, solver::Solver, Lit};
const EXTENSION: &str = "cnf";

#[derive(Serialize, Deserialize, Debug)]
pub struct FormulaDetails {
    num_variables: usize,
    num_clauses: usize,
    counts: Vec<LiteralCounts>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteralCounts {
    positive: usize,
    negative: usize,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn select_formula() -> FormulaDetails {
    let foo = select(Some(EXTENSION), Some(".."));
    use tauri_api::dialog::Response::Okay;
    let choice = if let Ok(Okay(file_name)) = foo {
        file_name
    } else {
        return FormulaDetails {
            num_clauses: 0,
            num_variables: 0,
            counts: vec![],
        };
    };

    let dimacs_cnf = fs::read(&choice).unwrap();
    let mut parser = dimacs::DimacsParser::new();
    parser.parse_chunk(dimacs_cnf.as_slice()).unwrap();
    let cnf_formula = parser.take_formula();
    let mut literal_counts = vec![
        LiteralCounts {
            positive: 0,
            negative: 0,
        };
        cnf_formula.var_count()
    ];

    for clause in cnf_formula.iter() {
        for literal in clause {
            let index = literal.index();
            if literal.is_positive() {
                literal_counts[index].positive = literal_counts[index].positive + 1;
            } else {
                literal_counts[index].negative = literal_counts[index].negative + 1;
            }
        }
    }

    FormulaDetails {
        num_clauses: cnf_formula.len(),
        num_variables: cnf_formula.var_count(),
        counts: literal_counts,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_formula])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// For a formula:
// Count how many times a literal appears, positive or negative
// Return an array:
// [ 1: { negative: n1, positive: n2 } ]
