use ex_4::{heap_sort, sample_data};

fn main() {
    let mut a = sample_data();

    println!("堆排序前:");
    for data in &a {
        print!("{:4}", data);
    }
    println!();

    heap_sort(&mut a);

    println!("堆排序后:");
    for data in &a {
        print!("{:4}", data);
    }
    println!();
}
