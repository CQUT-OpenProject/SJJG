use queue_ex_3::{PatientQueue, simulate_hospital};

#[test]
fn patient_queue_keeps_fifo_order() {
    let mut queue = PatientQueue::new();

    queue.enqueue("甲");
    queue.enqueue("乙");
    queue.enqueue("丙");

    assert_eq!(queue.output(), vec!["甲", "乙", "丙"]);
    assert_eq!(queue.dequeue(), Some("甲".to_string()));
    assert_eq!(queue.dequeue(), Some("乙".to_string()));
    assert_eq!(queue.len(), 1);
}

#[test]
fn hospital_simulation_handles_arrival_and_next() {
    let commands = [
        "arrive 张三",
        "arrive 李四",
        "next",
        "status",
        "next",
        "next",
    ];
    let result = simulate_hospital(&commands);

    assert_eq!(result[0], "病人 张三 到达，当前等待: [\"张三\"]");
    assert_eq!(result[1], "病人 李四 到达，当前等待: [\"张三\", \"李四\"]");
    assert_eq!(result[2], "病人 张三 进入诊室");
    assert_eq!(result[3], "剩余等待人数: 1，队列: [\"李四\"]");
    assert_eq!(result[4], "病人 李四 进入诊室");
    assert_eq!(result[5], "当前没有病人等待");
}
