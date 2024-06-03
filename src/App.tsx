import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { PrimeReactProvider } from 'primereact/api';

import { FormulaDetails } from "./models";
import Counts from "./Counts";

function App() {
  const [formula, setFormula] = useState({ num_variables: 0, num_clauses: 0 } as FormulaDetails);

  function clearFormula() {
    setFormula({
      file_name: "",
      num_clauses: 0,
      num_variables: 0,
      counts: [],
    });

  }
  async function select_formula() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setFormula(await invoke("select_formula", {}));
  }

  async function expandFormula() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setFormula(await invoke("expand_formula", { fileName: formula.file_name }));
  }

  let content = Counts(formula);
  let exp = <button onClick={expandFormula}>Expand</button>;

  return (
    <PrimeReactProvider>
      <div className="container">
        <h1>SAT Formula Viewer</h1>

        <p>Select a formula in DIMACS format with .cnf extension from your file system.</p>
        <div className="row">
          <button onClick={select_formula}>Select</button>
          <button onClick={clearFormula}>Clear</button>
          {formula.num_clauses > 0 ? exp : <></>}
        </div>
      </div>
      {content}
    </PrimeReactProvider >
  );
}

// To render a component
// 

export default App;
