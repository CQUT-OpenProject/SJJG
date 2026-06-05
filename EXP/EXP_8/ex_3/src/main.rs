use ex_3::{merge_sort, sample_data, shell_sort};

fn print_data(name: &str, a: &[i32]) {
    print!("{:<8}", name);
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
    shell_sort(&mut a);
    print_data("shell", &a);

    let mut a = data;
    merge_sort(&mut a);
    print_data("merge", &a);
}
