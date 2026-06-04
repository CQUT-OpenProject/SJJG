use std::collections::VecDeque;

/// AOE 网中的边（活动）
#[derive(Debug, Clone)]
pub struct Edge {
    /// 活动名称
    pub name: String,
    /// 起始事件编号
    pub from: usize,
    /// 终止事件编号
    pub to: usize,
    /// 活动持续时间
    pub duration: u32,
}

impl Edge {
    pub fn new(name: &str, from: usize, to: usize, duration: u32) -> Self {
        Self {
            name: name.to_string(),
            from,
            to,
            duration,
        }
    }
}

/// AOE 网（顶点表示事件，边表示活动）
#[derive(Debug, Clone)]
pub struct AoeNet {
    n: usize,
    pub edges: Vec<Edge>,
}

impl AoeNet {
    pub fn new(n: usize, edges: Vec<Edge>) -> Self {
        Self { n, edges }
    }

    /// 拓扑排序，返回顶点列表的拓扑次序
    fn topological_sort(&self) -> Option<Vec<usize>> {
        let mut in_degree = vec![0; self.n];

        for e in &self.edges {
            in_degree[e.to] += 1;
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        for v in 0..self.n {
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }

        let mut order = Vec::new();

        while let Some(v) = queue.pop_front() {
            order.push(v);

            for e in &self.edges {
                if e.from == v {
                    in_degree[e.to] -= 1;
                    if in_degree[e.to] == 0 {
                        queue.push_back(e.to);
                    }
                }
            }
        }

        if order.len() == self.n {
            Some(order)
        } else {
            None
        }
    }

    /// 计算关键路径
    /// 返回 (ve, vl, 关键活动列表, 总工期)
    pub fn critical_path(&self) -> Option<(Vec<u32>, Vec<u32>, Vec<String>, u32)> {
        let order = self.topological_sort()?;

        let mut ve = vec![0u32; self.n];

        for &v in &order {
            for e in &self.edges {
                if e.from == v {
                    let new_val = ve[v] + e.duration;
                    if new_val > ve[e.to] {
                        ve[e.to] = new_val;
                    }
                }
            }
        }

        let total_time = ve[self.n - 1];

        let mut vl = vec![total_time; self.n];

        for &v in order.iter().rev() {
            for e in &self.edges {
                if e.to == v {
                    let new_val = vl[v].saturating_sub(e.duration);
                    if new_val < vl[e.from] {
                        vl[e.from] = new_val;
                    }
                }
            }
        }

        let mut critical_activities: Vec<String> = Vec::new();

        for e in &self.edges {
            let ee = ve[e.from];
            let el = vl[e.to] - e.duration;

            if ee == el {
                critical_activities.push(e.name.clone());
            }
        }

        Some((ve, vl, critical_activities, total_time))
    }

    /// 计算每项活动的最早开始时间、最晚开始时间、浮动时间
    pub fn activity_times(&self) -> Vec<(String, u32, u32, u32)> {
        let mut result = Vec::new();

        if let Some((ve, vl, _, _)) = self.critical_path() {
            for e in &self.edges {
                let ee = ve[e.from];
                let el = vl[e.to] - e.duration;
                let slack = el - ee;
                result.push((e.name.clone(), ee, el, slack));
            }
        }

        result
    }

    pub fn event_count(&self) -> usize {
        self.n
    }

    pub fn activity_count(&self) -> usize {
        self.edges.len()
    }
}

/// 构建家庭宴会筹备 AOE 网
/// 顶点 0 为源点（开始），顶点 n-1 = 6 为汇点（开宴）
pub fn build_banquet_aoe() -> AoeNet {
    AoeNet::new(
        7,
        vec![
            Edge::new("A 菜单制定", 0, 1, 30),
            Edge::new("B 原料采购", 1, 2, 60),
            Edge::new("C 餐具准备", 1, 3, 45),
            Edge::new("D 甜点准备", 2, 4, 60),
            Edge::new("E 原料清洗", 2, 4, 60),
            Edge::new("F 烹饪", 4, 5, 30),
            Edge::new("G 桌椅布置", 3, 5, 15),
            Edge::new("H 宴会开始", 5, 6, 0),
        ],
    )
}
