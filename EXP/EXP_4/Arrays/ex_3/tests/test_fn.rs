use array_ex_3::{SparseMatrix, Triple};

#[test]
fn builds_triples_from_array_and_restores_array() {
    let matrix = SparseMatrix::from_array(vec![vec![1, 0, 2], vec![0, 3, 0]]);

    assert_eq!(
        matrix.triples(),
        vec![
            Triple {
                row: 0,
                col: 0,
                value: 1
            },
            Triple {
                row: 0,
                col: 2,
                value: 2
            },
            Triple {
                row: 1,
                col: 1,
                value: 3
            }
        ]
    );
    assert_eq!(matrix.to_array(), vec![vec![1, 0, 2], vec![0, 3, 0]]);
}

#[test]
fn adds_and_subtracts_sparse_matrices() {
    let a = SparseMatrix::from_array(vec![vec![1, 0, 2], vec![0, 3, 0]]);
    let b = SparseMatrix::from_array(vec![vec![0, 4, 0], vec![5, 0, 6]]);

    assert_eq!(
        a.add(&b).unwrap().to_array(),
        vec![vec![1, 4, 2], vec![5, 3, 6]]
    );
    assert_eq!(
        a.sub(&b).unwrap().to_array(),
        vec![vec![1, -4, 2], vec![-5, 3, -6]]
    );
}

#[test]
fn multiplies_sparse_matrices() {
    let a = SparseMatrix::from_array(vec![vec![1, 0, 2], vec![0, 3, 0]]);
    let b = SparseMatrix::from_array(vec![vec![1, 2], vec![0, 3], vec![4, 0]]);

    assert_eq!(a.mul(&b).unwrap().to_array(), vec![vec![9, 2], vec![0, 9]]);
}

#[test]
fn rejects_bad_matrix_sizes() {
    let a = SparseMatrix::from_array(vec![vec![1, 2]]);
    let b = SparseMatrix::from_array(vec![vec![1, 2, 3]]);

    assert!(a.add(&b).is_none());
    assert!(a.sub(&b).is_none());
}
