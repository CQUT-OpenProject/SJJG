use array_ex_3::SparseMatrix;

fn main() {
    let a = SparseMatrix::from_array(vec![vec![1, 0, 2], vec![0, 3, 0]]);
    let b = SparseMatrix::from_array(vec![vec![0, 4, 0], vec![5, 0, 6]]);
    let c = SparseMatrix::from_array(vec![vec![1, 2], vec![0, 3], vec![4, 0]]);

    println!("矩阵 A: {:?}", a.to_array());
    println!("矩阵 B: {:?}", b.to_array());
    println!("A + B: {:?}", a.add(&b).unwrap().to_array());
    println!("A - B: {:?}", a.sub(&b).unwrap().to_array());
    println!("A * C: {:?}", a.mul(&c).unwrap().to_array());
}
