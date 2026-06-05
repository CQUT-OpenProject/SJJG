use ex_2::{bubble_sort, insert_sort, quick_sort, sample_data, select_sort};

fn print_data(name: &str, a: &[i32]) {
    print!("{:<10}", name);
    for data in a {
        print!("{:4}", data);
    }
    println!();
}

fn main() {
    let data = sample_data();

    println!("排序前:");
    print_data("data", &data);
    println!("排序后:");

    let mut a = data;
    insert_sort(&mut a);
    print_data("insert", &a);

    let mut a = data;
    select_sort(&mut a);
    print_data("select", &a);

    let mut a = data;
    bubble_sort(&mut a);
    print_data("bubble", &a);

    let mut a = data;
    quick_sort(&mut a);
    print_data("quick", &a);
}
