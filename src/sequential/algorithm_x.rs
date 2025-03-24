use std::collections::HashSet;

pub fn algorithm_x(matrix: Vec<HashSet<usize>>) -> Vec<Vec<usize>> {
    // Convert matrix to track original row indices
    let indexed_matrix = matrix.into_iter()
        .enumerate()
        .map(|(i, cols)| (i, cols))
        .collect();

    algorithm_x_helper(indexed_matrix)
}

fn algorithm_x_helper(a: Vec<(usize, HashSet<usize>)>) -> Vec<Vec<usize>> {
                // Level 0
//Step 1: If matrix is not empty then proceed
//Base case: If every row is empty then we have a valid exact cover
    if a.is_empty() {
        return vec![vec![]];
    }

//Step 2: Select first column which has minimum number of 1's, let's called it column C
    //Determine all column
    let max_col = a.iter()
        .flat_map(|(_, cols)| cols.iter())
        .max()
        .cloned()
        .unwrap_or(0);

    // Select column with minimum number of 1's
    let mut c = 0; // column c
    let mut min_size = usize::MAX;
    for col in 0..=max_col {
        let col_count = a.iter()
            .filter(|(_, cols)| cols.contains(&col))
            .count();
        if col_count > 0 && col_count < min_size {
            min_size = col_count;
            c = col;
        }
    }
//Step 3: Select rows which have 1 at colum C, let called this set of rows 'N'
    // Collect rows that cover the selected colum, c
    let mut n = Vec::new();
    for (idx, (_, cols)) in a.iter().enumerate() {
        if cols.contains(&c) {
            n.push(idx);
        }
    }
        // Level 1: Select one row in set of rows 'N', let called row 'S

    let mut solutions = Vec::new();
//Step 4: Select row S and add it to partial solution, the set of column that have 1 in it
    for &s in &n {
        let (s_row, s_cols) = &a[s];
//Step 5: Find rows that have 1's at partial solution of S then removed this rows.
        let mut new_matrix = Vec::new();
        for (i, (orig, row_cols)) in a.iter().enumerate() {
            if i != s && row_cols.is_disjoint(s_cols) {
                new_matrix.push((*orig, row_cols.clone()));
            }
        }

        // Recursively solve reduced matrix
        let mut sub_solutions = algorithm_x_helper(new_matrix);

        // Add current row to solutions
        for solution in &mut sub_solutions {
            solution.push(*s_row);
        }
        solutions.extend(sub_solutions);
    }

    solutions
}

