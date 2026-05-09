use ex_4::sample_tree;

fn main() {
    let tree = sample_tree();

    println!("A 的孩子结点: {:?}", tree.child_values('A').unwrap());
    println!("B 的孩子结点: {:?}", tree.child_values('B').unwrap());
    println!("D 的孩子结点: {:?}", tree.child_values('D').unwrap());
    println!("G 的孩子结点: {:?}", tree.child_values('G').unwrap());
}
