fn algorithm_x(A: Vec<HasSet<usize>>) -> ? {
        // Level 0
    //Step 1: If matrix is not empty then proceed
    if !A.isEmpty(){
        return;
    }
    //Step 2: Select first column which has minimum number of 1's, let's called it column C
    
    //Step 3: Select rows which have 1 at colum C, let called this set of rows 'N'
    
        // Level 1: Select one row in set of rows 'N', let called row 'S'
    //Step 4: Select row S and add it to partial solution, the set of column that have 1 in it
    //Step 5: Find rows that have 1's at partial solution of S then removed this rows.
    // Repeat step 1 
    // if it unsucessful. backtrack at level 0 and proceed with another row in set of rows 'N'
