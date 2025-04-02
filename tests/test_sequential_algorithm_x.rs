use solve::sequential::algorithm_x::algorithm_x;
use solve::sudoku::sudoku_to_exact_cover;
use solve::sudoku::print_sudoku;
use solve::sudoku::decode_solution;
use std::collections::HashSet;

#[test]
fn basic_algorithm_x_test1() {
    let matrix1 = vec![
        HashSet::from([0, 3, 6]),
        HashSet::from([0, 3]),
        HashSet::from([3, 4, 6]),
        HashSet::from([2, 4, 5]),
        HashSet::from([1, 2, 5, 6]),
        HashSet::from([1, 6]),
    ];
    let mut solution1 = algorithm_x(matrix1);
    for sol in &mut solution1 {
        sol.sort();
    }
    solution1.sort();
    //println!("solutions: {:?}", solution1);
    assert_eq!(solution1, vec![vec![1, 3, 5]]);
}
#[test]
fn basic_algorithm_x_test2() {
    let matrix2 = vec![
        HashSet::from([0, 1]),
        HashSet::from([2, 3]),
    ];
    let mut solution2 = algorithm_x(matrix2);
    for sol in &mut solution2 {
        sol.sort();
    }
    solution2.sort();
    //println!("Solutions: {:?}", solution2);
    assert_eq!(solution2, vec![vec![0, 1]]);
}

#[test]
fn basic_algorithm_x_test3() {
    let matrix3 = vec![
        HashSet::from([1, 4, 7]),
        HashSet::from([1, 4]),
        HashSet::from([4, 5, 7]),
        HashSet::from([3, 5, 6]),
        HashSet::from([2, 3, 6, 7]),
        HashSet::from([2, 7]),
    ];
    let mut solution3 = algorithm_x(matrix3);
    for sol in &mut solution3 {
        sol.sort();
    }
    solution3.sort();
    //println!("Solutions: {:?}", solution3);
    assert_eq!(solution3, vec![vec![1, 3, 5]]);
}
#[test]
fn basic_algorithm_x_test4() {
    let matrix4 = vec![
        HashSet::from([1, 4, 7]),
        HashSet::from([1, 4, 6]),
        HashSet::from([4, 5, 7]),
        HashSet::from([3, 5, 6]),
        HashSet::from([2, 3, 6, 7]),
        HashSet::from([2, 7]),
    ];
    let solution4 = algorithm_x(matrix4);
    //println!("Solutions: {:?}", solution4);
    assert!(solution4.is_empty());
}

#[test]
fn test_multiple_solutions() {
    // This puzzle has multiple solutions
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
    print_sudoku(&puzzle);

    let (matrix, mapping) = sudoku_to_exact_cover(puzzle);
    let solutions = algorithm_x(matrix);

    println!("\nFound {} solutions:", solutions.len());
    for (i, solution) in solutions.iter().enumerate() {
        let grid = decode_solution(solution, &mapping);
        println!("\nSolution {}:", i + 1);
        print_sudoku(&grid);
        assert!(is_valid_sudoku(&grid));
    }
}


pub fn is_valid_sudoku(grid: &[[u8; 9]; 9]) -> bool {
    // Check rows
    for row in grid {
        let mut seen = [false; 9];
        for &num in row {
            if num == 0 { return false; }
            if seen[(num-1) as usize] { return false; }
            seen[(num-1) as usize] = true;
        }
    }

    // Check columns
    for col in 0..9 {
        let mut seen = [false; 9];
        for row in 0..9 {
            let num = grid[row][col];
            if seen[(num-1) as usize] { return false; }
            seen[(num-1) as usize] = true;
        }
    }

    // Check 3x3 boxes
    for box_row in (0..9).step_by(3) {
        for box_col in (0..9).step_by(3) {
            let mut seen = [false; 9];
            for r in 0..3 {
                for c in 0..3 {
                    let num = grid[box_row + r][box_col + c];
                    if seen[(num-1) as usize] { return false; }
                    seen[(num-1) as usize] = true;
                }
            }
        }
    }

    true
}
