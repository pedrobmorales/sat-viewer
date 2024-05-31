import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function select_formula() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("select_formula", { name }));
  }

  return (
    <div className="container">
      <h1>SAT Formula Viewer</h1>

      <p>Select a formula in DIMACS format with .cnf extension from your file system.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          select_formula();
        }}
      >
        <button type="submit">Select Formula</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
