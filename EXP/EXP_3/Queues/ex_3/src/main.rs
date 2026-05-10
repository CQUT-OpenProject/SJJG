use queue_ex_3::simulate_hospital;

fn main() {
    let commands = [
        "arrive 张三",
        "arrive 李四",
        "arrive 王五",
        "next",
        "status",
        "next",
        "next",
        "next",
    ];

    for line in simulate_hospital(&commands) {
        println!("{}", line);
    }
}
