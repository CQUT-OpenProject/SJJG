use exp_3::{averages, process::Process, round_robin};

/// 打印调度甘特图与统计
fn run_case(label: &str, processes: Vec<Process>, quantum: u32) {
    let result = round_robin(processes, quantum);
    let (avg_turn, avg_weighted) = averages(&result);

    println!("\n========== {} ==========", label);
    println!("时间片 = {}", quantum);
    println!("\n-- 甘特图（执行片段） --");
    for s in &result.slices {
        println!("  [{}, {})  {}", s.start, s.end, s.name);
    }

    println!("\n-- 进程统计 --");
    println!("  进程 | 到达 | 总需 | 开始 | 完成 | 周转 | 带权周转");
    for p in &result.processes {
        println!(
            "  {:>3} | {:>3} | {:>3} | {:>3} | {:>3} | {:>3} | {:.3}",
            p.name,
            p.arrival,
            p.total,
            p.first_start.unwrap(),
            p.finish.unwrap(),
            p.turnaround(),
            p.weighted_turnaround()
        );
    }
    println!("  平均周转时间     = {:.3}", avg_turn);
    println!("  平均带权周转时间 = {:.3}", avg_weighted);
}

fn main() {
    // 教材例题：进程 A、B、C、D 在 0 时刻到达，运行时间 20/10/15/5
    let procs_q5 = vec![
        Process::new("A", 0, 20),
        Process::new("B", 0, 10),
        Process::new("C", 0, 15),
        Process::new("D", 0, 5),
    ];

    run_case("时间片 = 5", procs_q5.clone(), 5);
    run_case("时间片 = 1", procs_q5, 1);
}
