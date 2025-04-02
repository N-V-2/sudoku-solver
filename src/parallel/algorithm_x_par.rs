use std::collections::HashSet;
use rayon::prelude::*;
use crate::algorithm_x_common::algorithm_x_helper;

pub fn algorithm_x_parallel(matrix: Vec<HashSet<usize>>) -> Vec<Vec<usize>> {
    let all_columns: HashSet<_> = matrix.iter()
        .flat_map(|row| row.iter())
        .cloned()
        .collect();

    let indexed_matrix = matrix.into_iter()
        .enumerate()
        .collect();

    let solutions = std::sync::Mutex::new(Vec::new());
    algorithm_x_helper_parallel(indexed_matrix, all_columns, &solutions);
    solutions.into_inner().unwrap()
}

fn algorithm_x_helper_parallel(matrix: Vec<(usize, HashSet<usize>)>,remaining_columns: HashSet<usize>,solutions: &std::sync::Mutex<Vec<Vec<usize>>>,) {
    if remaining_columns.is_empty() {
        solutions.lock().unwrap().push(Vec::new());
        return;
    }

    let col = *remaining_columns.iter()
        .min_by_key(|&c| matrix.iter().filter(|(_, cols)| cols.contains(c)).count())
        .unwrap();

    // Parallel processing of candidate rows
    matrix.par_iter()
        .enumerate()
        .filter(|(_, (_, cols))| cols.contains(&col))
        .for_each(|(row_idx, (orig_row, row_cols))| {
            let mut new_remaining = remaining_columns.clone();
            for c in row_cols {
                new_remaining.remove(c);
            }

            let reduced_matrix = matrix.iter()
                .enumerate()
                .filter(|(i, (_, other_cols))| *i != row_idx && other_cols.is_disjoint(row_cols))
                .map(|(_, (orig, cols))| (*orig, cols.clone()))
                .collect();

            let mut local_solutions = Vec::new();
            algorithm_x_helper(reduced_matrix, new_remaining, &mut local_solutions);

            if !local_solutions.is_empty() {
                let mut solutions = solutions.lock().unwrap();
                for mut solution in local_solutions {
                    solution.push(*orig_row);
                    solutions.push(solution);
                }
            }
        });
}
