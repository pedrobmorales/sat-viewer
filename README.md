# SAT Viewer

This project is a viewer application for Boolean Satisfiability formulas, usually in Conjunctive Normal Form with exactly 3 literals per clause (3CNF-SAT).  This is my attempt at understanding what makes a formula unsatisfiable.  A formula is satisfiable unless it has an unsatisfiable core, that is, a subset of the formula that contains a contradiction.  This viewer allows users to peek at an analysis of the formula.  It is a work in progress.  It does simple things like loading a formula from a DIMACS file on disk and then counting how many times a variable appears in positive and negative literals.  

This application is implemented in Rust and TypeScript using Tauri and React.  

TODO: Installation instructions for Rust, Tauri and npm.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
