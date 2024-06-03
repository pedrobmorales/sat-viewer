# SAT Viewer

This project is a viewer application for Boolean Satisfiability formulas, usually in Conjunctive Normal Form with exactly 3 literals per clause (3CNF-SAT).  This is my attempt at understanding what makes a formula unsatisfiable.  A formula is satisfiable unless it has an unsatisfiable core, that is, a subset of the formula that contains a contradiction.  This viewer allows users to peek at an analysis of the formula.  It is a work in progress.  It does simple things like loading a formula from a DIMACS file on disk and then counting how many times a variable appears in positive and negative literals.  

This application is implemented in Rust and TypeScript using Tauri and React.  

## Recommended Setup

The documentation to install Tauri contains great information for installing Rust and Node, which are both required for Tauri.

- [Install Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)
- [VS Code](https://code.visualstudio.com/)
- [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 
- [rust-analyzer Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Running the Application

To run the application, it is sufficient to do:

```
npm install
npm run tauri dev
```
