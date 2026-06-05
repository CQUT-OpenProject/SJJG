use ex_1::{sample_data, sort_records};

fn main() {
    let mut r = sample_data();

    println!("快速排序前:");
    for data in &r {
        print!("{}:{} ", data.no, data.key);
    }
    println!();

    sort_records(&mut r);

    println!("快速排序后:");
    for data in &r {
        print!("{}:{} ", data.no, data.key);
    }
    println!();
}
