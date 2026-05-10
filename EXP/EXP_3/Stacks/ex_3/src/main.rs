use stack_ex_3::{from_slice, reverse_list, to_vec};

fn main() {
    let head = from_slice(&[1, 2, 3, 4, 5]);
    println!("逆置前: {:?}", to_vec(&head));

    let reversed = reverse_list(head);
    println!("逆置后: {:?}", to_vec(&reversed));
}
