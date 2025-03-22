use std::collections::HashSet;
use solve::sequential::algorithm_x::algorithm_x;

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
    let solution1 = algorithm_x(matrix1);
    let expected = vec![
        vec![1, 4, 2],
        vec![0, 3, 5],
    ];
    assert!(expected.contains(&solution1));
}

#[test]
fn test2() { //There's no solution
    let matrix2 = vec![
        HashSet::from([0, 1]),
        HashSet::from([2, 3]),
    ];
    let solution2 = algorithm_x(matrix2);
    assert!(solution2.is_empty());
}

