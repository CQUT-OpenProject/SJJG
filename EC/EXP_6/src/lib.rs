/// 有向图的邻接矩阵存储
#[derive(Debug, Clone)]
pub struct DiGraph {
    n: usize,
    matrix: Vec<Vec<i32>>,
}

impl DiGraph {
    /// 创建 n 个顶点的图（顶点编号 0..n-1）
    pub fn new(n: usize) -> Self {
        Self {
            n,
            matrix: vec![vec![0; n]; n],
        }
    }

    /// 添加一条有向边 i -> j
    pub fn add_edge(&mut self, i: usize, j: usize) {
        if i < self.n && j < self.n {
            self.matrix[i][j] = 1;
        }
    }

    /// 计算各顶点的入度
    pub fn in_degrees(&self) -> Vec<usize> {
        let mut deg = vec![0usize; self.n];

        for i in 0..self.n {
            for j in 0..self.n {
                if self.matrix[i][j] == 1 {
                    deg[j] += 1;
                }
            }
        }

        deg
    }

    /// 计算各顶点的出度
    pub fn out_degrees(&self) -> Vec<usize> {
        let mut deg = vec![0usize; self.n];

        for i in 0..self.n {
            for j in 0..self.n {
                if self.matrix[i][j] == 1 {
                    deg[i] += 1;
                }
            }
        }

        deg
    }

    /// 返回顶点数
    pub fn vertex_count(&self) -> usize {
        self.n
    }

    /// 返回边数
    pub fn edge_count(&self) -> usize {
        let mut count = 0;

        for i in 0..self.n {
            for j in 0..self.n {
                if self.matrix[i][j] == 1 {
                    count += 1;
                }
            }
        }

        count
    }
}

/// 入度排名的结果
pub struct RankResult {
    pub vertex: usize,
    pub in_degree: usize,
}

/// 找出入度最高的前 k 个顶点
pub fn find_top_k(graph: &DiGraph, k: usize) -> Vec<RankResult> {
    let degrees = graph.in_degrees();
    let mut indexed: Vec<(usize, usize)> = (0..graph.n).map(|i| (i, degrees[i])).collect();

    indexed.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let limit = k.min(indexed.len());

    indexed[..limit]
        .iter()
        .map(|&(v, d)| RankResult {
            vertex: v,
            in_degree: d,
        })
        .collect()
}

/// 从边列表文件读取有向图
/// 文件格式：每行 time1 time2 from_id to_id（空白分隔）
/// 自动推算顶点数 = max(id) + 1
pub fn read_edges(path: &str) -> std::io::Result<DiGraph> {
    let content = std::fs::read_to_string(path)?;
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut max_id: usize = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        if let (Ok(from), Ok(to)) =
            (parts[2].parse::<usize>(), parts[3].parse::<usize>())
        {
            edges.push((from, to));
            if from > max_id {
                max_id = from;
            }
            if to > max_id {
                max_id = to;
            }
        }
    }

    let n = max_id + 1;
    let mut graph = DiGraph::new(n);

    for (from, to) in edges {
        graph.add_edge(from, to);
    }

    Ok(graph)
}

/// 生成测试用数据集（模拟 9 个班级的学生接触网络）
/// 返回写入的文件路径
pub fn generate_sample_data(path: &str) -> std::io::Result<()> {
    use std::io::Write;

    let mut file = std::fs::File::create(path)?;
    let class_size = 20;
    let num_classes = 9;
    let total_students = num_classes * class_size;
    let mut edges: Vec<String> = Vec::new();

    for day in 0..5 {
        let base_time = day * 24 * 3600;

        for c in 0..num_classes {
            let start_id = c * class_size;
            let end_id = start_id + class_size;

            for i in start_id..end_id {
                let contacts_count = if i % 5 == 0 {
                    12
                } else if i % 5 == 1 {
                    8
                } else {
                    4
                };

                for _ in 0..contacts_count {
                    let j = start_id + ((i * 7 + day * 3) % class_size);
                    if i != j {
                        let t = base_time + (i * 37 + j * 53) % (24 * 3600);
                        edges.push(format!(
                            "{} {} {} {}",
                            t,
                            t + 20,
                            i,
                            j
                        ));
                    }
                }
            }

            if c < num_classes - 1 {
                let bridge_count = 5;
                for _ in 0..bridge_count {
                    let i = start_id + (day * 7) % class_size;
                    let j = ((c + 1) * class_size) + (day * 3) % class_size;
                    let t = base_time + (i * 101 + j * 97) % (24 * 3600);
                    edges.push(format!(
                        "{} {} {} {}",
                        t,
                        t + 20,
                        i,
                        j
                    ));
                }
            }
        }

        let big_v_count = 8;
        for b in 0..big_v_count {
            let hub = (b * 23) % total_students;
            for _ in 0..25 {
                let target = (hub + 1 + (day * 17 + b * 13)) % total_students;
                if hub != target {
                    let t = base_time + (hub * 89 + target * 43) % (24 * 3600);
                    edges.push(format!(
                        "{} {} {} {}",
                        t,
                        t + 20,
                        hub,
                        target
                    ));
                }
            }
        }
    }

    edges.sort();
    edges.dedup();

    for e in &edges {
        writeln!(file, "{}", e)?;
    }

    Ok(())
}
