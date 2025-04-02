use std::collections::{HashSet, HashMap};
//use crate::sequential::algorithm_x::algorithm_x;
//use crate::parallel::algorithm_x_parallel::algorithm_x_parallel;


const N: usize = 9; //sudoku size 9x9

pub fn sudoku_to_exact_cover(grid: [[u8; N]; N]) -> (Vec<HashSet<usize>>, HashMap<usize, (usize, usize, u8)>) {
    let mut matrix = Vec::new();
    let mut mapping = HashMap::new();

    for row in 0..N {
        for col in 0..N{
            let cell_value = grid[row][col];
            if cell_value !=0 {
                let d = (cell_value - 1) as usize;
                let row_set = encode_sudoku_row(row,col, d as usize);
                matrix.push(row_set);
                mapping.insert(matrix.len() - 1, (row,col, cell_value));
            } else {
                for d in 0..N {
                    let row_set = encode_sudoku_row(row, col,d);
                    matrix.push(row_set);
                    mapping.insert(matrix.len()-1, (row,col, (d+1) as u8));
                }
            }
        }
    }
    (matrix,mapping)
}


fn encode_sudoku_row(r: usize, c:usize, d:usize) -> HashSet<usize> {
    let mut constraints = HashSet::with_capacity(4);
    constraints.insert(r * N + c); // Cell constraint
    constraints.insert(N * N + r * N + d); // Row constraint
    constraints.insert(2 * N * N + c * N + d); // Column constraint
    constraints.insert(3 * N * N + ((r / 3) * 3 + c / 3) * N + d); // Box constraint
    constraints
}

pub fn decode_solution(solution: &[usize], mapping: &HashMap<usize, (usize, usize, u8)>) -> [[u8;N];N] {
    let mut grid = [[0;9];9] ;
    for &row_index in solution {
        if let Some(&(r,c,d)) = mapping.get(&row_index){
            grid[r][c] = d;
        }
    }
    grid
}


pub fn print_sudoku(grid: &[[u8; 9]; 9]) {
    println!("+-------+-------+-------+");
    for (i, row) in grid.iter().enumerate() {
        print!("| ");
        for (j, &num) in row.iter().enumerate() {
            if num == 0 {
                print!(". ");
            } else {
                print!("{} ", num);
            }
            if j % 3 == 2 {
                print!("| ");
            }
        }
        println!();
        if i % 3 == 2 {
            println!("+-------+-------+-------+");
        }
    }
}
