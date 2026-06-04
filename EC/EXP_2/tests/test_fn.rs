use exp_2::{color_non_recursive, color_recursive, default_map, is_valid_coloring, region_name};

/// 构造一条 K4 子图用于测试：4 个节点两两相邻
fn k4_map() -> [[u8; 34]; 34] {
    let mut m = [[0u8; 34]; 34];
    let nodes = [0, 1, 2, 3];
    for &i in &nodes {
        for &j in &nodes {
            if i != j {
                m[i][j] = 1;
            }
        }
    }
    m
}

/// 构造一条无冲突图（无边）
fn empty_map() -> [[u8; 34]; 34] {
    [[0u8; 34]; 34]
}

#[test]
fn recursive_4color_finds_solution() {
    let map = default_map();
    let c = color_recursive(&map, 4).expect("4 着色应可解");
    assert!(is_valid_coloring(&map, &c));
    // 颜色取值应只落在 1..=4
    for i in 0..34 {
        assert!((1..=4).contains(&c[i]), "颜色 {} 越界 at {}", c[i], i);
    }
}

#[test]
fn non_recursive_4color_matches_recursive_validity() {
    let map = default_map();
    let c1 = color_recursive(&map, 4).unwrap();
    let c2 = color_non_recursive(&map, 4).unwrap();
    assert!(is_valid_coloring(&map, &c1));
    assert!(is_valid_coloring(&map, &c2));
}

#[test]
fn three_color_should_fail() {
    let map = default_map();
    let result = color_recursive(&map, 3);
    assert!(result.is_none(), "3 着色对中国地图应无解");
    let result2 = color_non_recursive(&map, 3);
    assert!(result2.is_none());
}

#[test]
fn k4_needs_four_colors() {
    let map = k4_map();
    assert!(color_recursive(&map, 3).is_none());
    let c = color_recursive(&map, 4).unwrap();
    assert!(is_valid_coloring(&map, &c));
}

#[test]
fn empty_map_one_color_sufficient() {
    let map = empty_map();
    let c = color_recursive(&map, 1).unwrap();
    // 全为 1 即可
    for v in c.iter() {
        assert_eq!(*v, 1);
    }
    assert!(is_valid_coloring(&map, &c));
}

#[test]
fn region_names_count() {
    let map = default_map();
    let c = color_recursive(&map, 4).unwrap();
    for i in 0..34 {
        assert!(!region_name(i).is_empty());
        assert!(c[i] >= 1);
    }
}

#[test]
fn non_border_pairs_are_not_adjacent() {
    let map = default_map();
    let pairs = [
        (1, 14),  // 天津 - 山东
        (3, 14),  // 山西 - 山东
        (4, 30),  // 内蒙古 - 新疆
        (12, 31), // 福建 - 台湾
        (18, 20), // 广东 - 海南
        (19, 20), // 广西 - 海南
    ];

    for &(i, j) in pairs.iter() {
        assert_eq!(
            map[i][j],
            0,
            "{} 和 {} 不应相邻",
            region_name(i),
            region_name(j)
        );
        assert_eq!(
            map[j][i],
            0,
            "{} 和 {} 不应相邻",
            region_name(j),
            region_name(i)
        );
    }
}
