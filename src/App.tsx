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
      num_clauses: 0,
      num_variables: 0,
      counts: [],
    });

  }
  async function select_formula() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setFormula(await invoke("select_formula", {}));
  }

  let content = Counts(formula);

  console.log("Formula", formula);
  return (
    <PrimeReactProvider>
      <div className="container">
        <h1>SAT Formula Viewer</h1>

        <p>Select a formula in DIMACS format with .cnf extension from your file system.</p>
        <div className="row">
          <button onClick={select_formula}>Select Formula</button>
          <button onClick={clearFormula}>Clear Formula</button>
        </div>
      </div>
      {content}
    </PrimeReactProvider >
  );
}

export default App;
