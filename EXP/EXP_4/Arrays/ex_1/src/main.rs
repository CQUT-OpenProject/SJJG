use array_ex_1::move_non_zero_front;

fn main() {
    let mut data = vec![0, 3, 0, 8, 0, 2, 5];

    println!("移动前: {:?}", data);
    move_non_zero_front(&mut data);
    println!("移动后: {:?}", data);
}
