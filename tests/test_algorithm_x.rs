use solve::sequential::algorithm_x::algorithm_x;
use std::collections::HashSet;

#[test]
fn test1() {
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
    println!("solutions: {:?}", solution1);
    assert_eq!(solution1, vec![vec![1, 3, 5]]);
}
#[test]
fn test2() {
    let matrix2 = vec![
        HashSet::from([0, 1]),  
        HashSet::from([2, 3]),  
    ];
    let mut solution2 = algorithm_x(matrix2);
    for sol in &mut solution2 {  
        sol.sort();
    }
    solution2.sort();
    println!("Solutions: {:?}", solution2);
    assert_eq!(solution2, vec![vec![0, 1]]);
}

#[test]
fn test3() {
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
    println!("Solutions: {:?}", solution3);
    assert_eq!(solution3, vec![vec![1, 3, 5]]);
}
#[test]
fn test4() {
    let matrix4 = vec![
        HashSet::from([1, 4, 7]),
        HashSet::from([1, 4, 6]),
        HashSet::from([4, 5, 7]),
        HashSet::from([3, 5, 6]),
        HashSet::from([2, 3, 6, 7]),
        HashSet::from([2, 7]),
    ];
    let mut solution4 = algorithm_x(matrix4);
    println!("Solutions: {:?}", solution4);
    assert!(solution4.is_empty());
}


