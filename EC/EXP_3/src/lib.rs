pub mod process;
pub mod queue;

use crate::process::Process;
use crate::queue::Queue;

/// 时间片调度中的一次记录：(进程名, 开始时间, 结束时间)
#[derive(Clone, Debug)]
pub struct Slice {
    pub name: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Clone, Debug)]
pub struct ScheduleResult {
    pub slices: Vec<Slice>,
    pub processes: Vec<Process>,
}

/// 时间片轮转（Round Robin）调度算法
/// 输入：进程列表、时间片大小
/// 输出：调度过程与各进程的完成信息
pub fn round_robin(input: Vec<Process>, quantum: u32) -> ScheduleResult {
    assert!(quantum > 0, "时间片必须大于 0");

    let n = input.len();
    let mut q: Queue<usize> = Queue::new(n + 4);
    let mut procs = input;
    procs.sort_by_key(|p| p.arrival);
    let mut slices: Vec<Slice> = Vec::new();
    let mut now: u32 = 0;
    let mut i: usize = 0;
    let mut finished: usize = 0;

    if n == 0 {
        return ScheduleResult {
            slices,
            processes: procs,
        };
    }

    if procs[0].arrival > now {
        now = procs[0].arrival;
    }

    // 初始入队
    while i < n && procs[i].arrival <= now {
        q.enqueue(i);
        i += 1;
    }

    while !q.is_empty() {
        let idx = q.dequeue().unwrap();
        let start = now;

        if procs[idx].first_start.is_none() {
            procs[idx].first_start = Some(start);
        }

        if procs[idx].remaining <= quantum {
            // 一次执行完毕
            now += procs[idx].remaining;
            procs[idx].remaining = 0;
            procs[idx].finish = Some(now);
            slices.push(Slice {
                name: procs[idx].name.clone(),
                start,
                end: now,
            });
            finished += 1;
        } else {
            now += quantum;
            procs[idx].remaining -= quantum;
            slices.push(Slice {
                name: procs[idx].name.clone(),
                start,
                end: now,
            });
        }

        // 将调度过程中到达的进程入队
        while i < n && procs[i].arrival <= now {
            q.enqueue(i);
            i += 1;
        }

        // 当前进程未完成则再次入队
        if procs[idx].remaining > 0 {
            q.enqueue(idx);
        }

        // 队列空但还有未到达的进程：跳到下一进程的到达时间
        if q.is_empty() && finished < n {
            now = procs[i].arrival;
            while i < n && procs[i].arrival <= now {
                q.enqueue(i);
                i += 1;
            }
        }
    }

    ScheduleResult {
        slices,
        processes: procs,
    }
}

/// 计算平均周转时间和平均带权周转时间
pub fn averages(result: &ScheduleResult) -> (f64, f64) {
    let n = result.processes.len() as f64;
    let sum_turnaround: f64 = result.processes.iter().map(|p| p.turnaround() as f64).sum();
    let sum_weighted: f64 = result
        .processes
        .iter()
        .map(|p| p.weighted_turnaround())
        .sum();
    (sum_turnaround / n, sum_weighted / n)
}
