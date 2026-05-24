#[derive(Debug, Clone)]
pub struct MatrixGraph {
    // data 保存顶点值，下标就是邻接矩阵中的行号、列号。
    data: Vec<i32>,
    // lines[i][j] 为 1 表示顶点 i 和顶点 j 之间有边，为 0 表示没有边。
    lines: Vec<Vec<i32>>,
}

impl MatrixGraph {
    pub fn new(data: Vec<i32>) -> Self {
        let len = data.len();

        Self {
            data,
            // 无权图的邻接矩阵先全部置 0，后面插入边时再把对应位置置 1。
            lines: vec![vec![0; len]; len],
        }
    }

    fn find_pos(&self, key: i32) -> Option<usize> {
        let mut idx = 0;

        // 顶点数量不多，顺序查找更直观，也接近实验中的数组写法。
        while idx < self.data.len() {
            if self.data[idx] == key {
                return Some(idx);
            }
            idx += 1;
        }

        None
    }

    pub fn insert_line(&mut self, a: i32, b: i32) -> bool {
        // 先把顶点值转换成矩阵下标，找不到顶点则不能插入边。
        let i = match self.find_pos(a) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(b) {
            Some(pos) => pos,
            None => return false,
        };

        // 图 10-7 是无向图，所以矩阵中两个对称位置都要置 1。
        self.lines[i][j] = 1;
        self.lines[j][i] = 1;
        true
    }

    pub fn delete_line(&mut self, a: i32, b: i32) -> bool {
        let i = match self.find_pos(a) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(b) {
            Some(pos) => pos,
            None => return false,
        };

        // 原本没有这条边时，删除操作失败。
        if self.lines[i][j] == 0 {
            return false;
        }

        // 无向图删除边时，也要同时删除对称位置。
        self.lines[i][j] = 0;
        self.lines[j][i] = 0;
        true
    }

    pub fn has_line(&self, a: i32, b: i32) -> bool {
        let i = match self.find_pos(a) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(b) {
            Some(pos) => pos,
            None => return false,
        };

        self.lines[i][j] == 1
    }

    pub fn dfs(&self, start: i32) -> Vec<i32> {
        // visited[i] 表示第 i 个顶点是否已经被访问过。
        let mut visited = vec![false; self.data.len()];
        let mut result = Vec::new();

        if let Some(pos) = self.find_pos(start) {
            self.dfs_fun(pos, &mut visited, &mut result);
        }

        // 若图不连通，从还没有访问的点重新开始，直到全部顶点访问完成。
        let mut i = 0;
        while i < self.data.len() {
            if !visited[i] {
                self.dfs_fun(i, &mut visited, &mut result);
            }
            i += 1;
        }

        result
    }

    fn dfs_fun(&self, idx: usize, visited: &mut Vec<bool>, result: &mut Vec<i32>) {
        // 访问当前顶点，并把顶点值放入搜索序列。
        visited[idx] = true;
        result.push(self.data[idx]);

        // 按矩阵列号从小到大寻找邻接点，遇到未访问顶点就递归深入。
        let mut j = 0;
        while j < self.data.len() {
            if self.lines[idx][j] == 1 && !visited[j] {
                self.dfs_fun(j, visited, result);
            }
            j += 1;
        }
    }

    pub fn to_list_graph(&self) -> MatrixListGraph {
        let mut graph = MatrixListGraph::new(self.data.clone());

        let mut i = 0;
        while i < self.lines.len() {
            // 头插法会改变邻接点顺序，这里倒着扫描，输出时仍然接近矩阵顺序。
            let mut j = self.lines[i].len();
            while j > 0 {
                j -= 1;
                if self.lines[i][j] == 1 {
                    graph.insert_line_by_pos(i, j);
                }
            }
            i += 1;
        }

        graph
    }

    pub fn output(&self) -> String {
        let mut result = String::new();

        // 第一行输出顶点标号，方便观察邻接矩阵的行列含义。
        result.push_str("  ");
        for v in &self.data {
            result.push_str(&format!("{} ", v));
        }
        result.push('\n');

        let mut i = 0;
        while i < self.lines.len() {
            result.push_str(&format!("{} ", self.data[i]));

            let mut j = 0;
            while j < self.lines[i].len() {
                result.push_str(&format!("{} ", self.lines[i][j]));
                j += 1;
            }

            result.push('\n');
            i += 1;
        }

        result
    }
}

pub fn build_sample_graph() -> MatrixGraph {
    let mut graph = MatrixGraph::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);

    graph.insert_line(0, 1);
    graph.insert_line(0, 5);
    graph.insert_line(1, 2);
    graph.insert_line(1, 4);
    graph.insert_line(2, 3);
    graph.insert_line(3, 4);
    graph.insert_line(5, 6);
    graph.insert_line(5, 7);
    graph.insert_line(6, 7);

    graph
}

#[derive(Debug, Clone)]
struct ListLineNode {
    // adjvex 保存邻接点在顶点数组中的下标。
    adjvex: usize,
    // next 指向同一个顶点的下一条边。
    next: Option<Box<ListLineNode>>,
}

#[derive(Debug, Clone)]
struct ListVexNode {
    data: i32,
    // first 指向该顶点的第一条边。
    first: Option<Box<ListLineNode>>,
}

#[derive(Debug, Clone)]
pub struct MatrixListGraph {
    vexs: Vec<ListVexNode>,
}

impl MatrixListGraph {
    pub fn new(data: Vec<i32>) -> Self {
        let mut vexs = Vec::new();

        for v in data {
            vexs.push(ListVexNode {
                data: v,
                first: None,
            });
        }

        Self { vexs }
    }

    fn insert_line_by_pos(&mut self, i: usize, j: usize) {
        // 把矩阵中的一条边转换成邻接表结点，使用头插法连接到链表前面。
        let node = Box::new(ListLineNode {
            adjvex: j,
            next: self.vexs[i].first.take(),
        });
        self.vexs[i].first = Some(node);
    }

    pub fn output(&self) -> String {
        let mut result = String::new();
        let mut i = 0;

        while i < self.vexs.len() {
            result.push_str(&format!("{}:", self.vexs[i].data));
            let mut curr = self.vexs[i].first.as_ref();

            while let Some(node) = curr {
                result.push_str(&format!(" -> {}", self.vexs[node.adjvex].data));
                curr = node.next.as_ref();
            }

            result.push('\n');
            i += 1;
        }

        result
    }
}

const INF: i32 = 9999;

#[derive(Debug, Clone)]
pub struct WeightedMatrixGraph {
    data: Vec<i32>,
    // 带权图矩阵保存边权，INF 表示两个顶点之间没有直接边。
    lines: Vec<Vec<i32>>,
}

impl WeightedMatrixGraph {
    pub fn new(data: Vec<i32>) -> Self {
        let len = data.len();
        let mut lines = vec![vec![INF; len]; len];

        // 顶点到自己的距离记为 0。
        let mut i = 0;
        while i < len {
            lines[i][i] = 0;
            i += 1;
        }

        Self { data, lines }
    }

    fn find_pos(&self, key: i32) -> Option<usize> {
        let mut idx = 0;

        while idx < self.data.len() {
            if self.data[idx] == key {
                return Some(idx);
            }
            idx += 1;
        }

        None
    }

    pub fn insert_line(&mut self, a: i32, b: i32, weight: i32) -> bool {
        let i = match self.find_pos(a) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(b) {
            Some(pos) => pos,
            None => return false,
        };

        self.lines[i][j] = weight;
        self.lines[j][i] = weight;
        true
    }

    pub fn prim(&self, start: i32) -> Vec<(i32, i32, i32)> {
        let start_pos = match self.find_pos(start) {
            Some(pos) => pos,
            None => return Vec::new(),
        };

        let len = self.data.len();
        let mut lowcost = vec![INF; len];
        let mut adjvex = vec![start_pos; len];
        let mut visited = vec![false; len];
        let mut result = Vec::new();

        // 从指定顶点开始，先把它放入最小生成树集合。
        visited[start_pos] = true;
        let mut i = 0;
        while i < len {
            // lowcost[i] 记录当前生成树到顶点 i 的最小边权。
            lowcost[i] = self.lines[start_pos][i];
            // adjvex[i] 记录这条最小边来自哪个顶点。
            adjvex[i] = start_pos;
            i += 1;
        }

        let mut count = 1;
        while count < len {
            let mut min = INF;
            let mut k: Option<usize> = None;
            let mut j = 0;

            // 在还没有加入生成树的顶点中，找 lowcost 最小的那个顶点。
            while j < len {
                if !visited[j] && lowcost[j] < min {
                    min = lowcost[j];
                    k = Some(j);
                }
                j += 1;
            }

            let pos = match k {
                Some(value) => value,
                None => break,
            };

            visited[pos] = true;
            result.push((self.data[adjvex[pos]], self.data[pos], lowcost[pos]));

            // 新顶点加入后，尝试用它更新其它顶点到生成树的最短连接边。
            let mut m = 0;
            while m < len {
                if !visited[m] && self.lines[pos][m] < lowcost[m] {
                    lowcost[m] = self.lines[pos][m];
                    adjvex[m] = pos;
                }
                m += 1;
            }

            count += 1;
        }

        result
    }
}

pub fn build_weighted_sample_graph() -> WeightedMatrixGraph {
    let mut graph = WeightedMatrixGraph::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);

    graph.insert_line(0, 1, 4);
    graph.insert_line(0, 5, 3);
    graph.insert_line(1, 2, 5);
    graph.insert_line(1, 4, 2);
    graph.insert_line(2, 3, 6);
    graph.insert_line(3, 4, 3);
    graph.insert_line(5, 6, 7);
    graph.insert_line(5, 7, 8);
    graph.insert_line(6, 7, 1);

    graph
}
