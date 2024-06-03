type FormulaDetails = {
    file_name: string,
    num_variables: number,
    num_clauses: number,
    counts: LiteralCounts[],


}
type LiteralCounts = {
    positive: number,
    negative: number,
}

export type { FormulaDetails, LiteralCounts };