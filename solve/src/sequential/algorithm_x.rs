use std::collections::HashSet;
fn algorithm_x(a: Vec<HashSet<usize>>) -> Vec<Vec<usize>> {
        // Level 0
    //Step 1: If matrix is not empty then proceed
    if a.is_empty(){
        return vec![vec![]];
    }
    //Step 2: Select first column which has minimum number of 1's, let's called it column C
    let mut min_col = None;
    let mut min_size = usize::MAX;
    for (col,rows) in a.iter().enumerate(){
        if rows.len() < min_size {
            min_size = rows.len();
            min_col = Some(col);
        }
    }
    let c = min_col.unwrap();
    //Step 3: Select rows which have 1 at colum C, let called this set of rows 'N'
    let  mut n = Vec::new();
    for (row_index, row) in a.iter().enumerate() {
        if row.contains(&c) {
            n.push(row_index);
        }
    }
        // Level 1: Select one row in set of rows 'N', let called row 'S'
    let mut solutions = Vec::new();
    //Step 4: Select row S and add it to partial solution, the set of column that have 1 in it
    for &s in &n {
        let mut a_new = a.clone();
    //Step 5: Find rows that have 1's at partial solution of S then removed this rows.
        let remove = a[s].clone();
        a_new.retain(|row| row.is_disjoint(&remove));
    // Repeat step 1 
        let sub_solutions = algorithm_x(a_new); 
        // if it unsucessful. backtrack at level 0 and proceed with another row in set of rows 'N'
        for mut solution in sub_solutions{
            solution.push(s);
            solutions.push(solution);
        }
    }
    solutions
}
