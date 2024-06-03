import { FormulaDetails } from "./models";

function Counts(formula: FormulaDetails) {
    let content;
    if (formula.num_clauses != 0) {
        content = <>
            <p>Vars: {formula.num_variables} Clauses: {formula.num_clauses}</p>
            <ol>
                {formula.counts.length > 0 && formula.counts.map((index, count) => (
                    <li key={count}>{index.negative} negative and {index.positive} positive</li>
                ))
                }
            </ol>
        </>
    } else {
        content = <div>Yes null formula</div>
    }

    return content;
}

export default Counts;