use exp_2::{color_non_recursive, color_recursive, default_map, is_valid_coloring, region_name};

fn print_coloring(label: &str, color: &[u8; 34]) {
    println!("\n=== {} ===", label);
    for i in 0..34 {
        println!("  {:>4} -> 色 {}", region_name(i), color[i]);
    }
}

fn main() {
    let map = default_map();

    // 递归 4 着色
    match color_recursive(&map, 4) {
        Some(c) => {
            assert!(is_valid_coloring(&map, &c));
            print_coloring("递归 4 着色方案", &c);
        }
        None => println!("递归：未找到 4 着色方案"),
    }

    // 非递归 4 着色
    match color_non_recursive(&map, 4) {
        Some(c) => {
            assert!(is_valid_coloring(&map, &c));
            print_coloring("非递归 4 着色方案", &c);
        }
        None => println!("非递归：未找到 4 着色方案"),
    }

    // 3 着色可行性验证
    let three = color_recursive(&map, 3);
    match three {
        Some(_) => println!("\n3 着色：存在合法方案（与四色定理预期不符）"),
        None => println!("\n3 着色：无合法方案，验证通过——中国地图不能只用 3 种颜色"),
    }
}
