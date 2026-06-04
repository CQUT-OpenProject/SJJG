use exp_3::{averages, process::Process, round_robin};

fn classic_four() -> Vec<Process> {
    vec![
        Process::new("A", 0, 20),
        Process::new("B", 0, 10),
        Process::new("C", 0, 15),
        Process::new("D", 0, 5),
    ]
}

#[test]
fn quantum_5_matches_textbook() {
    let result = round_robin(classic_four(), 5);
    // 期望：4 个进程均完成
    assert_eq!(result.processes.len(), 4);
    for p in &result.processes {
        assert_eq!(p.remaining, 0);
    }

    // 各进程完成时间
    let by_name = |n: &str| {
        result
            .processes
            .iter()
            .find(|p| p.name == n)
            .unwrap()
            .finish
            .unwrap()
    };
    assert_eq!(by_name("A"), 50);
    assert_eq!(by_name("B"), 30);
    assert_eq!(by_name("C"), 45);
    assert_eq!(by_name("D"), 20);

    // 平均指标
    let (avg_t, avg_w) = averages(&result);
    assert!((avg_t - 36.25).abs() < 1e-9);
    assert!((avg_w - 3.125).abs() < 1e-9);
}

#[test]
fn quantum_1_matches_textbook() {
    let result = round_robin(classic_four(), 1);
    for p in &result.processes {
        assert_eq!(p.remaining, 0);
    }
    let by_name = |n: &str| {
        result
            .processes
            .iter()
            .find(|p| p.name == n)
            .unwrap()
            .finish
            .unwrap()
    };
    assert_eq!(by_name("A"), 50);
    assert_eq!(by_name("B"), 34);
    assert_eq!(by_name("C"), 45);
    assert_eq!(by_name("D"), 20);

    let (avg_t, avg_w) = averages(&result);
    assert!((avg_t - 37.25).abs() < 1e-9);
    assert!((avg_w - 3.225).abs() < 1e-9);
}

#[test]
fn total_runtime_equals_work() {
    let result = round_robin(classic_four(), 3);
    // 总执行时间应当等于所有进程服务时间之和
    let total: u32 = result.processes.iter().map(|p| p.total).sum();
    let makespan = result
        .processes
        .iter()
        .map(|p| p.finish.unwrap())
        .max()
        .unwrap();
    assert_eq!(makespan, total);
}

#[test]
fn staggered_arrivals() {
    // A 在 0 时刻到达需要 5ms，B 在 3 时刻到达需要 2ms，时间片 = 2
    // 调度序列：[0,2)A, [2,4)A, [4,6)B, [6,7)A
    // 期望：A 完成于 7，B 完成于 6；makespan = 7
    let procs = vec![Process::new("A", 0, 5), Process::new("B", 3, 2)];
    let result = round_robin(procs, 2);
    let by_name = |n: &str| {
        result
            .processes
            .iter()
            .find(|p| p.name == n)
            .unwrap()
            .finish
            .unwrap()
    };
    assert_eq!(by_name("A"), 7);
    assert_eq!(by_name("B"), 6);
}

#[test]
fn quantum_too_large_runs_to_completion() {
    // 时间片远大于所有进程服务时间时，进程依次完成
    let procs = vec![Process::new("X", 0, 3), Process::new("Y", 0, 4)];
    let result = round_robin(procs, 100);
    let by_name = |n: &str| {
        result
            .processes
            .iter()
            .find(|p| p.name == n)
            .unwrap()
            .finish
            .unwrap()
    };
    assert_eq!(by_name("X"), 3);
    assert_eq!(by_name("Y"), 7);
}

#[test]
fn first_process_can_arrive_after_zero() {
    let procs = vec![Process::new("A", 5, 3)];
    let result = round_robin(procs, 2);

    assert_eq!(result.slices.len(), 2);
    assert_eq!(result.slices[0].start, 5);
    assert_eq!(result.slices[0].end, 7);
    assert_eq!(result.slices[1].start, 7);
    assert_eq!(result.slices[1].end, 8);
    assert_eq!(result.processes[0].finish, Some(8));
}

#[test]
fn input_is_sorted_by_arrival_time() {
    let procs = vec![Process::new("B", 3, 2), Process::new("A", 0, 5)];
    let result = round_robin(procs, 2);
    let by_name = |n: &str| {
        result
            .processes
            .iter()
            .find(|p| p.name == n)
            .unwrap()
            .finish
            .unwrap()
    };

    assert_eq!(by_name("A"), 7);
    assert_eq!(by_name("B"), 6);
}

#[test]
#[should_panic(expected = "时间片必须大于 0")]
fn quantum_zero_is_invalid() {
    let _ = round_robin(classic_four(), 0);
}
