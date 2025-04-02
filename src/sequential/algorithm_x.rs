use std::collections::HashSet;
//use crate::algorithm_x_common::algorithm_x_helper as other_algorithm_x_helper;
pub fn algorithm_x(matrix: Vec<HashSet<usize>>) -> Vec<Vec<usize>> {
    // Get all column
    let all_columns = matrix.iter().flat_map(|row| row.iter().cloned()).collect();
    // Convert matrix to track original row indices
    let indexed_matrix = matrix.into_iter()
        .enumerate()
        .map(|(i, cols)| (i, cols))
        .collect();
    let mut solutions = Vec::new();
    algorithm_x_helper(indexed_matrix, all_columns, &mut solutions);
    solutions
}

fn algorithm_x_helper(matrix: Vec<(usize, HashSet<usize>)>, remaining_columns: HashSet<usize>,solutions: &mut Vec<Vec<usize>>,){
                // Level 0
//Step 1: If matrix is not empty then proceed
//Base case: All columns covered 
    if remaining_columns.is_empty() {
        solutions.push(Vec::new());
        return;
    }

//Step 2: Select column which has minimum number of 1's, let's called it column C
    let c = *remaining_columns.iter()
        .min_by_key(|&&col| matrix.iter()
            .filter(|(_,cols)| cols.contains(&col))
            .count())
        .unwrap();
    
//Step 3: Select rows which have 1 at colum C, let called this set of rows 'N'
    // Collect rows that cover the selected colum, c
    for (row_index, (original_row, row_cols)) in matrix.iter().enumerate()
        .filter(|(_, (_, cols))| cols.contains(&c))
    {
        // Create remaining columns by removing covered column
        let mut new_remaining = remaining_columns.clone();
        for col in row_cols {
            new_remaining.remove(col);
        }
        //Step 5: Find rows that have 1's at partial solution of S then removed this rows.
        let reduced_matrix = matrix.iter()
            .enumerate()
            .filter(|(i, (_, other_cols))| 
                *i != row_index && other_cols.is_disjoint(row_cols))
            .map(|(_, (orig, cols))| (*orig, cols.clone()))
            .collect();
        // Recursively solve reduced problem
        let prev_len = solutions.len();
        algorithm_x_helper(reduced_matrix, new_remaining, solutions);

        for solution in solutions.iter_mut().skip(prev_len) {
            solution.push(*original_row);
        }

    }
}

