use exp_7::{to_html, InvertedIndex};

#[test]
fn build_index_from_dir() {
    let index = InvertedIndex::build_from_dir("docs").unwrap();
    assert!(index.term_count() > 0);
}

#[test]
fn search_found() {
    let index = InvertedIndex::build_from_dir("docs").unwrap();
    let results = index.search("mars");
    assert!(!results.is_empty());
    assert!(results.iter().any(|d| d == "mars"));
}

#[test]
fn search_not_found() {
    let index = InvertedIndex::build_from_dir("docs").unwrap();
    let results = index.search("xyznonexistent");
    assert!(results.is_empty());
}

#[test]
fn search_case_insensitive() {
    let index = InvertedIndex::build_from_dir("docs").unwrap();
    let lower = index.search("data");
    let upper = index.search("DATA");
    assert_eq!(lower, upper);
}

#[test]
fn html_output() {
    let results = vec!["animals".to_string(), "weather".to_string()];
    let html = to_html("snow", &results);
    assert!(html.contains("MARS"));
    assert!(html.contains("snow"));
    assert!(html.contains("animals"));
    assert!(html.contains("2 个相关文档"));
}

#[test]
fn html_no_results() {
    let html = to_html("nothing", &[]);
    assert!(html.contains("未找到"));
}

#[test]
fn empty_index_search() {
    let index = InvertedIndex::new();
    let results = index.search("anything");
    assert!(results.is_empty());
    assert_eq!(index.term_count(), 0);
}
