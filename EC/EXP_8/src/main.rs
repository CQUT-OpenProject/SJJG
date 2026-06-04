use exp_8::build_banquet_aoe;

fn main() {
    let aoe = build_banquet_aoe();

    println!("=== 家庭宴会筹备 · 关键路径分析 ===");
    println!("事件数: {}", aoe.event_count());
    println!("活动数: {}", aoe.activity_count());
    println!();

    if let Some((ve, vl, critical, total)) = aoe.critical_path() {
        println!("总工期: {} 分钟", total);
        println!();

        println!("=== 事件的最早/最晚开始时间 ===");
        println!("{:<8} {:<12} {:<12}", "事件", "ve (最早)", "vl (最晚)");
        for v in 0..aoe.event_count() {
            println!("  v{:<6} {:<12} {:<12}", v, ve[v], vl[v]);
        }
        println!();

        let times = aoe.activity_times();
        println!("=== 活动时间分析 ===");
        println!(
            "{:<22} {:<6} {:<8} {:<8} {:<8} {:<6}",
            "活动", "耗时", "EE", "EL", "Slack", "关键"
        );
        for (name, ee, el, slack) in &times {
            let is_critical = if *slack == 0 { "☆" } else { "" };
            println!(
                "{:<22} {:<6} {:<8} {:<8} {:<8} {:<6}",
                name,
                aoe.edges
                    .iter()
                    .find(|e| &e.name == name)
                    .map(|e| e.duration)
                    .unwrap_or(0),
                ee,
                el,
                slack,
                is_critical,
            );
        }
        println!();

        println!("关键活动: {}", critical.join(" → "));

        let p = vec![
            "v0(开始)",
            "v1(菜单完成)",
            "v2(采购完成)",
            "v4(甜点和清洗完成)",
            "v5(烹饪和布置完成)",
            "v6(宴会开始)",
        ];
        println!();
        println!("=== 关键路径追踪 ===");
        println!("{}", p.join(" → "));
        println!("  耗时: 30+60+60+30+0 = {} 分钟", 30 + 60 + 60 + 30);
    } else {
        println!("错误：AOE 网中存在环，无法进行关键路径分析");
    }
}
