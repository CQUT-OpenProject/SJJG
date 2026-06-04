use exp_8::{AoeNet, Edge, build_banquet_aoe};

#[test]
fn build_aoe_network() {
    let aoe = build_banquet_aoe();
    assert_eq!(aoe.event_count(), 7);
    assert_eq!(aoe.activity_count(), 8);
}

#[test]
fn topological_sort_valid() {
    let aoe = build_banquet_aoe();
    assert!(aoe.critical_path().is_some());
}

#[test]
fn critical_path_not_empty() {
    let aoe = build_banquet_aoe();
    let (_, _, critical, total) = aoe.critical_path().unwrap();
    assert!(!critical.is_empty());
    assert!(total > 0);
}

#[test]
fn total_time_correct() {
    let aoe = build_banquet_aoe();
    let (_, _, _, total) = aoe.critical_path().unwrap();
    assert_eq!(total, 180);
}

#[test]
fn activity_times_slack_zero_for_critical() {
    let aoe = build_banquet_aoe();
    let (_, _, critical, _) = aoe.critical_path().unwrap();
    let times = aoe.activity_times();

    for (name, _ee, _el, slack) in &times {
        if critical.contains(name) {
            assert_eq!(*slack, 0, "关键活动 {} slack 应为 0", name);
        }
    }
}

#[test]
fn ve_non_decreasing() {
    let aoe = build_banquet_aoe();
    let (ve, vl, _, _) = aoe.critical_path().unwrap();

    for e in &aoe.edges {
        assert!(
            ve[e.from] + e.duration <= ve[e.to],
            "边 {}->{}: ve[{}]={} + dur={} 应 <= ve[{}]={}",
            e.from,
            e.to,
            e.from,
            ve[e.from],
            e.duration,
            e.to,
            ve[e.to]
        );
    }

    for e in &aoe.edges {
        assert!(
            vl[e.from] <= vl[e.to].saturating_sub(e.duration),
            "边 {}->{}: vl[{}]={} 应 <= vl[{}]={} - dur={}",
            e.from,
            e.to,
            e.from,
            vl[e.from],
            e.to,
            vl[e.to],
            e.duration
        );
    }
}

#[test]
fn table_setting_is_not_critical() {
    let aoe = build_banquet_aoe();
    let (_, _, critical, _) = aoe.critical_path().unwrap();
    assert!(!critical.contains(&"G 桌椅布置".to_string()));
}

#[test]
fn banquet_critical_activities_match_requirement() {
    let aoe = build_banquet_aoe();
    let (_, _, critical, total) = aoe.critical_path().unwrap();

    assert_eq!(total, 180);
    assert!(critical.contains(&"A 菜单制定".to_string()));
    assert!(critical.contains(&"B 原料采购".to_string()));
    assert!(critical.contains(&"D 甜点准备".to_string()));
    assert!(critical.contains(&"E 原料清洗".to_string()));
    assert!(critical.contains(&"F 烹饪".to_string()));
    assert!(critical.contains(&"H 宴会开始".to_string()));
}

#[test]
fn single_edge_graph() {
    let aoe = AoeNet::new(2, vec![Edge::new("A", 0, 1, 10)]);
    let (ve, _vl, critical, total) = aoe.critical_path().unwrap();
    assert_eq!(total, 10);
    assert_eq!(ve[0], 0);
    assert_eq!(ve[1], 10);
    assert_eq!(critical, vec!["A"]);
}
