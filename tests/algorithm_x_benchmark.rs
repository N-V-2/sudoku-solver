use solve::sequential::algorithm_x::algorithm_x as sequential;
use solve::parallel::algorithm_x_par::algorithm_x_parallel as parallel;
use solve::sudoku::{sudoku_to_exact_cover, print_sudoku, decode_solution};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
fn run_benchmark<F>(name: &str, f: F, matrix: Vec<HashSet<usize>>, expected: Vec<Vec<usize>>) 
where
    F: Fn(Vec<HashSet<usize>>) -> Vec<Vec<usize>>,
{
    let start = Instant::now();
    let mut solutions = f(matrix);
    let duration = start.elapsed();
    
    // Normalize solutions for comparison
    for sol in &mut solutions {
        sol.sort();
    }
    solutions.sort();
    
    assert_eq!(solutions, expected, "Incorrect solutions for {}", name);
    println!("{}: {:?} ({} solutions)", name, duration, solutions.len());
}

#[test]
fn compare_implementations() {
    let test_cases = vec![
        (
            "Test 1", 
            vec![
                HashSet::from([0, 3, 6]),
                HashSet::from([0, 3]),
                HashSet::from([3, 4, 6]),
                HashSet::from([2, 4, 5]),
                HashSet::from([1, 2, 5, 6]),
                HashSet::from([1, 6]),
            ],
            vec![vec![1, 3, 5]]
        ),
        (
            "Test 2",
            vec![
                HashSet::from([0, 1]),
                HashSet::from([2, 3]),
            ],
            vec![vec![0, 1]]
        ),
        (
            "Test 3",
            vec![
                HashSet::from([1, 4, 7]),
                HashSet::from([1, 4]),
                HashSet::from([4, 5, 7]),
                HashSet::from([3, 5, 6]),
                HashSet::from([2, 3, 6, 7]),
                HashSet::from([2, 7]),
            ],
            vec![vec![1, 3, 5]]
        ),
        (
            "Test 4",
            vec![
                HashSet::from([1, 4, 7]),
                HashSet::from([1, 4, 6]),
                HashSet::from([4, 5, 7]),
                HashSet::from([3, 5, 6]),
                HashSet::from([2, 3, 6, 7]),
                HashSet::from([2, 7]),
            ],
            vec![]
        ),
    ];
    
    for (name, matrix, expected) in test_cases {
        let matrix_clone = matrix.clone();
        let expected_clone = expected.clone();
        
        println!("\nRunning {}:", name);
        run_benchmark("Sequential", sequential, matrix, expected);
        run_benchmark("Parallel", parallel, matrix_clone, expected_clone);
    }
}
fn print_solution(solution: &[usize], mapping: &HashMap<usize, (usize, usize, u8)>) {
    let grid = decode_solution(solution, mapping);
    print_sudoku(&grid);
}

#[test]
fn sudoku_2solution() {
    let puzzle = [
        [2, 9, 5, 7, 4, 3, 8, 6, 1],
        [4, 3, 1, 8, 6, 5, 9, 0, 0],
        [8, 7, 6, 1, 9, 2, 5, 4, 3],
        [3, 8, 7, 4, 5, 9, 2, 1, 6],
        [6, 1, 2, 3, 8, 7, 4, 9, 5],
        [5, 4, 9, 2, 1, 6, 7, 3, 8],
        [7, 6, 3, 5, 2, 4, 1, 8, 9],
        [9, 2, 8, 6, 7, 1, 3, 5, 4],
        [1, 5, 4, 9, 3, 8, 6, 0, 0],
    ];

    println!("Original Sudoku:");
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    
    // Sequential Solution
    println!("\nSequential Solutions:");
    let seq_start = Instant::now();
    let seq_solutions = sequential(matrix.clone());
    let seq_time = seq_start.elapsed();
    
    println!("Found {} solutions in {:?}:", seq_solutions.len(), seq_time);
    for (i, sol) in seq_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Parallel Solution
    println!("\nParallel Solutions:");
    let par_start = Instant::now();
    let par_solutions = parallel(matrix.clone());
    let par_time = par_start.elapsed();
    
    println!("Found {} solutions in {:?}:", par_solutions.len(), par_time);
    for (i, sol) in par_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Print timing comparison without asserting equality
    println!("\nPerformance Summary:");
    println!("Sequential time: {:?} ({} solutions)", seq_time, seq_solutions.len());
    println!("Parallel time:   {:?} ({} solutions)", par_time, par_solutions.len());
    if !par_solutions.is_empty() && !seq_solutions.is_empty() {
        println!("Speedup:         {:.2}x", 
                seq_time.as_secs_f64() / par_time.as_secs_f64());
    }
}

#[test]
fn sudoku_4solution() {
    let puzzle = [
      [0, 0, 5, 7, 4, 3, 8, 6, 1],
      // Pair 2: row2, columns 8–9 (originally [2,7]) have been blanked.
      [4, 3, 1, 8, 6, 5, 9, 0, 0],
      [8, 7, 6, 1, 9, 2, 5, 4, 3],
      [3, 8, 7, 4, 5, 9, 2, 1, 6],
      [6, 1, 2, 3, 8, 7, 4, 9, 5],
      [5, 4, 9, 2, 1, 6, 7, 3, 8],
      [7, 6, 3, 5, 2, 4, 1, 8, 9],
      // Pair 1 continued: row8, columns 1–2 (originally [9,2]) have been blanked.
      [0, 0, 8, 6, 7, 1, 3, 5, 4],
      // Pair 2 continued: row9, columns 8–9 (originally [7,2]) have been blanked.
      [1, 5, 4, 9, 3, 8, 6, 0, 0],
    ];

    println!("Original Sudoku:");
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    
    // Sequential Solution
    println!("\nSequential Solutions:");
    let seq_start = Instant::now();
    let seq_solutions = sequential(matrix.clone());
    let seq_time = seq_start.elapsed();
    
    println!("Found {} solutions in {:?}:", seq_solutions.len(), seq_time);
    for (i, sol) in seq_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Parallel Solution
    println!("\nParallel Solutions:");
    let par_start = Instant::now();
    let par_solutions = parallel(matrix.clone());
    let par_time = par_start.elapsed();
    
    println!("Found {} solutions in {:?}:", par_solutions.len(), par_time);
    for (i, sol) in par_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Print timing comparison without asserting equality
    println!("\nPerformance Summary:");
    println!("Sequential time: {:?} ({} solutions)", seq_time, seq_solutions.len());
    println!("Parallel time:   {:?} ({} solutions)", par_time, par_solutions.len());
    if !par_solutions.is_empty() && !seq_solutions.is_empty() {
        println!("Speedup:         {:.2}x", 
                seq_time.as_secs_f64() / par_time.as_secs_f64());
    }
}
//parallel x1.74 time faster
#[test]
fn weird_sudoku() {
    let puzzle = [
        [0, 0, 0, 0, 0, 0, 0, 1, 2],
        [0, 0, 0, 0, 0, 0, 0, 0, 3],
        [0, 0, 2, 3, 0, 0, 4, 0, 0],
        [0, 0, 1, 8, 0, 0, 0, 0, 5],
        [0, 6, 0, 0, 7, 0, 8, 0, 0],
        [0, 0, 0, 0, 0, 9, 0, 0, 0],
        [0, 0, 8, 5, 0, 0, 0, 0, 0],
        [9, 0, 0, 0, 4, 0, 5, 0, 0],
        [4, 7, 0, 0, 0, 6, 0, 0, 0],
    ];

    println!("Original Sudoku:");
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    
    // Sequential Solution
    println!("\nSequential Solutions:");
    let seq_start = Instant::now();
    let seq_solutions = sequential(matrix.clone());
    let seq_time = seq_start.elapsed();
    
    println!("Found {} solutions in {:?}:", seq_solutions.len(), seq_time);
    for (i, sol) in seq_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Parallel Solution
    println!("\nParallel Solutions:");
    let par_start = Instant::now();
    let par_solutions = parallel(matrix.clone());
    let par_time = par_start.elapsed();
    
    println!("Found {} solutions in {:?}:", par_solutions.len(), par_time);
    for (i, sol) in par_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Print timing comparison without asserting equality
    println!("\nPerformance Summary:");
    println!("Sequential time: {:?} ({} solutions)", seq_time, seq_solutions.len());
    println!("Parallel time:   {:?} ({} solutions)", par_time, par_solutions.len());
    if !par_solutions.is_empty() && !seq_solutions.is_empty() {
        println!("Speedup:         {:.2}x", 
                seq_time.as_secs_f64() / par_time.as_secs_f64());
    }
}

#[test]
fn sudoku_easy() {
    let puzzle = [
        [0, 0, 2, 0, 0, 1, 6, 0, 7],
        [0, 6, 8, 7, 0, 0, 9, 0, 0],
        [1, 0, 0, 3, 6, 8, 0, 5, 4],
        [0, 0, 0, 0, 0, 3, 4, 7, 9],
        [7, 0, 4, 0, 0, 0, 0, 0, 2],
        [0, 1, 0, 0, 7, 6, 0, 0, 0],
        [0, 5, 0, 0, 3, 0, 0, 0, 0],
        [9, 2, 0, 1, 0, 0, 3, 0, 5],
        [4, 7, 3, 0, 2, 5, 8, 0, 1],
    ];

    println!("Original Sudoku:");
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    
    // Sequential Solution
    println!("\nSequential Solutions:");
    let seq_start = Instant::now();
    let seq_solutions = sequential(matrix.clone());
    let seq_time = seq_start.elapsed();
    
    println!("Found {} solutions in {:?}:", seq_solutions.len(), seq_time);
    for (i, sol) in seq_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Parallel Solution
    println!("\nParallel Solutions:");
    let par_start = Instant::now();
    let par_solutions = parallel(matrix.clone());
    let par_time = par_start.elapsed();
    
    println!("Found {} solutions in {:?}:", par_solutions.len(), par_time);
    for (i, sol) in par_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Print timing comparison without asserting equality
    println!("\nPerformance Summary:");
    println!("Sequential time: {:?} ({} solutions)", seq_time, seq_solutions.len());
    println!("Parallel time:   {:?} ({} solutions)", par_time, par_solutions.len());
    if !par_solutions.is_empty() && !seq_solutions.is_empty() {
        println!("Speedup:         {:.2}x", 
                seq_time.as_secs_f64() / par_time.as_secs_f64());
    }
}
#[test]
fn sudoku_expert() {
    let puzzle = [
        [0, 0, 0, 4, 3, 5, 1, 7, 9],
        [5, 0, 3, 0, 0, 0, 4, 0, 0],
        [0, 0, 0, 0, 0, 0, 3, 0, 0],
        [0, 0, 0, 0, 2, 6, 7, 0, 1],
        [0, 1, 0, 0, 0, 0, 0, 2, 4],
        [0, 2, 9, 1, 7, 0, 0, 0, 0],
        [0, 5, 0, 8, 0, 3, 2, 1, 0],
        [0, 0, 4, 0, 1, 9, 0, 0, 3],
        [0, 3, 0, 0, 6, 0, 0, 0, 0],
    ];

    println!("Original Sudoku:");
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    
    // Sequential Solution
    println!("\nSequential Solutions:");
    let seq_start = Instant::now();
    let seq_solutions = sequential(matrix.clone());
    let seq_time = seq_start.elapsed();
    
    println!("Found {} solutions in {:?}:", seq_solutions.len(), seq_time);
    for (i, sol) in seq_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Parallel Solution
    println!("\nParallel Solutions:");
    let par_start = Instant::now();
    let par_solutions = parallel(matrix.clone());
    let par_time = par_start.elapsed();
    
    println!("Found {} solutions in {:?}:", par_solutions.len(), par_time);
    for (i, sol) in par_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Print timing comparison without asserting equality
    println!("\nPerformance Summary:");
    println!("Sequential time: {:?} ({} solutions)", seq_time, seq_solutions.len());
    println!("Parallel time:   {:?} ({} solutions)", par_time, par_solutions.len());
    if !par_solutions.is_empty() && !seq_solutions.is_empty() {
        println!("Speedup:         {:.2}x", 
                seq_time.as_secs_f64() / par_time.as_secs_f64());
    }
}

#[test]
fn sudoku_master() {
    let puzzle = [
        [6, 0, 2, 0, 0, 0, 0, 7, 0],
        [0, 0, 0, 0, 0, 6, 0, 5, 1],
        [0, 7, 0, 0, 0, 5, 9, 0, 0],
        [8, 0, 0, 0, 6, 0, 7, 0, 0],
        [2, 0, 0, 4, 7, 0, 0, 8, 0],
        [4, 6, 7, 5, 3, 0, 0, 0, 0],
        [7, 2, 4, 0, 0, 0, 0, 0, 8],
        [0, 0, 9, 0, 0, 7, 3, 0, 0],
        [0, 0, 0, 2, 0, 0, 1, 0, 0],
    ];

    println!("Original Sudoku:");
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    
    // Sequential Solution
    println!("\nSequential Solutions:");
    let seq_start = Instant::now();
    let seq_solutions = sequential(matrix.clone());
    let seq_time = seq_start.elapsed();
    
    println!("Found {} solutions in {:?}:", seq_solutions.len(), seq_time);
    for (i, sol) in seq_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Parallel Solution
    println!("\nParallel Solutions:");
    let par_start = Instant::now();
    let par_solutions = parallel(matrix.clone());
    let par_time = par_start.elapsed();
    
    println!("Found {} solutions in {:?}:", par_solutions.len(), par_time);
    for (i, sol) in par_solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        print_solution(sol, &mapping);
    }
    
    // Print timing comparison without asserting equality
    println!("\nPerformance Summary:");
    println!("Sequential time: {:?} ({} solutions)", seq_time, seq_solutions.len());
    println!("Parallel time:   {:?} ({} solutions)", par_time, par_solutions.len());
    if !par_solutions.is_empty() && !seq_solutions.is_empty() {
        println!("Speedup:         {:.2}x", 
                seq_time.as_secs_f64() / par_time.as_secs_f64());
    }
}
