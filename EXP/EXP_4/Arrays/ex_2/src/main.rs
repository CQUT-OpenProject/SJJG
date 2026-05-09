use array_ex_2::odd_before_even;

fn main() {
    let mut data = vec![2, 9, 4, 7, 6, 3, 8, 1];

    println!("调整前: {:?}", data);
    odd_before_even(&mut data);
    println!("调整后: {:?}", data);
}
