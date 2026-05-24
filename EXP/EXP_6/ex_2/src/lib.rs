#[derive(Debug, Clone)]
struct LineNode {
    // adjvex 是弧头顶点在顶点数组中的下标。
    adjvex: usize,
    // next 连接同一个弧尾顶点的下一条出边。
    next: Option<Box<LineNode>>,
}

#[derive(Debug, Clone)]
struct VexNode {
    data: i32,
    // first 指向该顶点的第一条出边。
    first: Option<Box<LineNode>>,
}

#[derive(Debug, Clone)]
pub struct ListGraph {
    vexs: Vec<VexNode>,
}

impl ListGraph {
    pub fn new(data: Vec<i32>) -> Self {
        let mut vexs = Vec::new();

        // 先建立顶点表，每个顶点的边链表开始时都是空的。
        for v in data {
            vexs.push(VexNode {
                data: v,
                first: None,
            });
        }

        Self { vexs }
    }

    fn find_pos(&self, key: i32) -> Option<usize> {
        let mut idx = 0;

        // 通过顶点值寻找顶点在数组中的位置。
        while idx < self.vexs.len() {
            if self.vexs[idx].data == key {
                return Some(idx);
            }
            idx += 1;
        }

        None
    }

    pub fn insert_line(&mut self, from: i32, to: i32) -> bool {
        // 有向图插入的是 from -> to 这一条边。
        let i = match self.find_pos(from) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(to) {
            Some(pos) => pos,
            None => return false,
        };

        if self.has_line(from, to) {
            return false;
        }

        // 使用头插法，把新边结点接到 from 顶点边链表的最前面。
        let node = Box::new(LineNode {
            adjvex: j,
            next: self.vexs[i].first.take(),
        });
        self.vexs[i].first = Some(node);
        true
    }

    pub fn delete_line(&mut self, from: i32, to: i32) -> bool {
        let i = match self.find_pos(from) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(to) {
            Some(pos) => pos,
            None => return false,
        };

        let mut curr = &mut self.vexs[i].first;

        // 在 from 的出边链表中逐个查找 to，找到后把当前结点摘掉。
        while curr.is_some() {
            if curr.as_ref().unwrap().adjvex == j {
                let next = curr.as_mut().unwrap().next.take();
                *curr = next;
                return true;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }

        false
    }

    pub fn has_line(&self, from: i32, to: i32) -> bool {
        let i = match self.find_pos(from) {
            Some(pos) => pos,
            None => return false,
        };
        let j = match self.find_pos(to) {
            Some(pos) => pos,
            None => return false,
        };

        let mut curr = self.vexs[i].first.as_ref();

        // 只检查 from 的出边，所以 1->2 和 2->1 是两种不同情况。
        while let Some(node) = curr {
            if node.adjvex == j {
                return true;
            }
            curr = node.next.as_ref();
        }

        false
    }

    pub fn dfs(&self, start: i32) -> Vec<i32> {
        // visited[i] 记录第 i 个顶点是否访问过。
        let mut visited = vec![false; self.vexs.len()];
        let mut result = Vec::new();

        if let Some(pos) = self.find_pos(start) {
            self.dfs_fun(pos, &mut visited, &mut result);
        }

        // 有向图可能不连通，未访问过的顶点还要继续遍历。
        let mut i = 0;
        while i < self.vexs.len() {
            if !visited[i] {
                self.dfs_fun(i, &mut visited, &mut result);
            }
            i += 1;
        }

        result
    }

    pub fn bfs(&self, start: i32) -> Vec<i32> {
        let mut visited = vec![false; self.vexs.len()];
        let mut result = Vec::new();

        // 先从指定顶点开始广度优先搜索。
        if let Some(pos) = self.find_pos(start) {
            self.bfs_fun(pos, &mut visited, &mut result);
        }

        // 图 10-8 不是强连通图，一次遍历后还要从未访问顶点继续。
        let mut i = 0;
        while i < self.vexs.len() {
            if !visited[i] {
                self.bfs_fun(i, &mut visited, &mut result);
            }
            i += 1;
        }

        result
    }

    fn bfs_fun(&self, idx: usize, visited: &mut Vec<bool>, result: &mut Vec<i32>) {
        // queue 保存待访问顶点下标，head 指向当前出队位置。
        let mut queue = Vec::new();
        let mut head = 0;

        // 顶点入队时就标记，防止同一个顶点重复入队。
        visited[idx] = true;
        queue.push(idx);

        while head < queue.len() {
            let pos = queue[head];
            head += 1;
            result.push(self.vexs[pos].data);

            // 依次检查当前顶点的所有邻接点，未访问的顶点放入队尾。
            let mut curr = self.vexs[pos].first.as_ref();
            while let Some(node) = curr {
                if !visited[node.adjvex] {
                    visited[node.adjvex] = true;
                    queue.push(node.adjvex);
                }
                curr = node.next.as_ref();
            }
        }
    }

    fn dfs_fun(&self, idx: usize, visited: &mut Vec<bool>, result: &mut Vec<i32>) {
        visited[idx] = true;
        result.push(self.vexs[idx].data);

        // 沿着当前顶点的出边链表向下查找，能深入就递归深入。
        let mut curr = self.vexs[idx].first.as_ref();
        while let Some(node) = curr {
            if !visited[node.adjvex] {
                self.dfs_fun(node.adjvex, visited, result);
            }
            curr = node.next.as_ref();
        }
    }

    pub fn topo_sort(&self) -> Vec<i32> {
        let mut indegree = vec![0; self.vexs.len()];
        let mut i = 0;

        // 统计每个顶点的入度：每看到一条 i -> j，就让 j 的入度加 1。
        while i < self.vexs.len() {
            let mut curr = self.vexs[i].first.as_ref();
            while let Some(node) = curr {
                indegree[node.adjvex] += 1;
                curr = node.next.as_ref();
            }
            i += 1;
        }

        let mut stack = Vec::new();
        let mut j = 0;
        while j < indegree.len() {
            // 入度为 0 的顶点可以先输出。
            if indegree[j] == 0 {
                stack.push(j);
            }
            j += 1;
        }

        let mut result = Vec::new();
        while let Some(pos) = stack.pop() {
            result.push(self.vexs[pos].data);

            // 删除 pos 发出的边：相邻顶点入度减 1，减到 0 就入栈。
            let mut curr = self.vexs[pos].first.as_ref();
            while let Some(node) = curr {
                indegree[node.adjvex] -= 1;
                if indegree[node.adjvex] == 0 {
                    stack.push(node.adjvex);
                }
                curr = node.next.as_ref();
            }
        }

        result
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

pub fn build_sample_graph() -> ListGraph {
    let mut graph = ListGraph::new(vec![1, 2, 3, 4, 5, 6]);

    graph.insert_line(1, 2);
    graph.insert_line(1, 3);
    graph.insert_line(1, 4);
    graph.insert_line(3, 2);
    graph.insert_line(3, 5);
    graph.insert_line(4, 5);
    graph.insert_line(6, 4);
    graph.insert_line(6, 5);

    graph
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: i32,
    pub to: i32,
    pub weight: i32,
}

#[derive(Debug, Clone)]
pub struct EdgeGraph {
    vexs: Vec<i32>,
    // 边集数组中每个元素保存一条边的两个端点和权值。
    edges: Vec<Edge>,
}

impl EdgeGraph {
    pub fn new(vexs: Vec<i32>) -> Self {
        Self {
            vexs,
            edges: Vec::new(),
        }
    }

    fn find_pos(&self, key: i32) -> Option<usize> {
        let mut idx = 0;

        while idx < self.vexs.len() {
            if self.vexs[idx] == key {
                return Some(idx);
            }
            idx += 1;
        }

        None
    }

    pub fn insert_line(&mut self, from: i32, to: i32, weight: i32) -> bool {
        if self.find_pos(from).is_none() || self.find_pos(to).is_none() {
            return false;
        }

        self.edges.push(Edge { from, to, weight });
        true
    }

    fn get_end(parent: &Vec<usize>, mut idx: usize) -> usize {
        // 沿着 parent 数组向上找，直到找到集合的最终代表。
        while parent[idx] != idx {
            idx = parent[idx];
        }

        idx
    }

    pub fn kruskal(&self) -> Vec<Edge> {
        let mut edges = self.edges.clone();
        let mut i = 0;

        // 按边权从小到大排序，方便依次选择最短边。
        while i < edges.len() {
            let mut j = i + 1;
            while j < edges.len() {
                if edges[i].weight > edges[j].weight {
                    edges.swap(i, j);
                }
                j += 1;
            }
            i += 1;
        }

        let mut parent = Vec::new();
        let mut k = 0;
        while k < self.vexs.len() {
            // 开始时每个顶点各自属于一个集合。
            parent.push(k);
            k += 1;
        }

        let mut result = Vec::new();
        for e in edges {
            let from_pos = self.find_pos(e.from).unwrap();
            let to_pos = self.find_pos(e.to).unwrap();
            let m = Self::get_end(&parent, from_pos);
            let n = Self::get_end(&parent, to_pos);

            // 两个端点不在同一集合，说明加入这条边不会形成回路。
            if m != n {
                parent[m] = n;
                result.push(e);
            }
        }

        result
    }
}

pub fn build_edge_sample_graph() -> EdgeGraph {
    let mut graph = EdgeGraph::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);

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
