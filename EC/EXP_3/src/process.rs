/// 进程信息
#[derive(Clone, Debug)]
pub struct Process {
    pub name: String,
    pub arrival: u32,
    pub total: u32,
    pub remaining: u32,
    pub first_start: Option<u32>,
    pub finish: Option<u32>,
}

impl Process {
    pub fn new(name: &str, arrival: u32, total: u32) -> Self {
        Process {
            name: name.to_string(),
            arrival,
            total,
            remaining: total,
            first_start: None,
            finish: None,
        }
    }

    /// 周转时间
    pub fn turnaround(&self) -> u32 {
        self.finish.unwrap_or(0).saturating_sub(self.arrival)
    }

    /// 带权周转时间（保留两位小数的浮点）
    pub fn weighted_turnaround(&self) -> f64 {
        let t = self.turnaround() as f64;
        t / self.total as f64
    }
}
