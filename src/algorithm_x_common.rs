use std::collections::HashSet;

pub fn algorithm_x_helper(
    matrix: Vec<(usize, HashSet<usize>)>,
    remaining_columns: HashSet<usize>,
    solutions: &mut Vec<Vec<usize>>,
) {
    if remaining_columns.is_empty() {
        solutions.push(Vec::new());
        return;
    }

    let col = *remaining_columns.iter()
        .min_by_key(|&c| matrix.iter().filter(|(_, cols)| cols.contains(c)).count())
        .unwrap();

    for (row_idx, (orig_row, row_cols)) in matrix.iter()
        .enumerate()
        .filter(|(_, (_, cols))| cols.contains(&col))
    {
        let mut new_remaining = remaining_columns.clone();
        for c in row_cols {
            new_remaining.remove(c);
        }

        let reduced_matrix = matrix.iter()
            .enumerate()
            .filter(|(i, (_, other_cols))| *i != row_idx && other_cols.is_disjoint(row_cols))
            .map(|(_, (orig, cols))| (*orig, cols.clone()))
            .collect();

        let prev_len = solutions.len();
        algorithm_x_helper(reduced_matrix, new_remaining, solutions);

        for solution in solutions.iter_mut().skip(prev_len) {
            solution.push(*orig_row);
        }
    }
}
