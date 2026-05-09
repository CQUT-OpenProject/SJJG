use std::fs::File;
use std::io::{BufRead, BufReader};

use ex_3::{Point, Triangle};

fn main() -> std::io::Result<()> {
    let file = File::open("inputfile.txt")?;
    let reader = BufReader::new(file);

    let mut nums = Vec::new();
    for line in reader.lines() {
        for word in line?.split_whitespace() {
            if let Ok(v) = word.parse::<f64>() {
                nums.push(v);
            }
        }
    }

    if nums.len() < 8 {
        println!("输入数据不足");
        return Ok(());
    }

    let triangle = Triangle::new(
        Point::new(nums[0], nums[1]),
        Point::new(nums[2], nums[3]),
        Point::new(nums[4], nums[5]),
    );
    let p = Point::new(nums[6], nums[7]);

    if !triangle.is_valid() {
        println!("三点不能构成有效三角形");
        return Ok(());
    }

    println!("是否等边三角形: {}", triangle.is_equilateral());
    println!("是否等腰三角形: {}", triangle.is_isosceles());
    println!("是否直角三角形: {}", triangle.is_right());
    println!("三角形面积: {:.6}", triangle.area());
    println!("点是否在三角形内部: {}", triangle.contains_point(p));

    Ok(())
}
