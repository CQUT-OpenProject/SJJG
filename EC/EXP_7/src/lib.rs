use std::collections::{HashMap, HashSet};

/// 倒排索引：词 → 包含该词的文档名集合
pub struct InvertedIndex {
    index: HashMap<String, Vec<String>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// 从一个目录中读取所有 .txt 文件并构建倒排索引
    pub fn build_from_dir(dir: &str) -> std::io::Result<Self> {
        let mut idx = Self::new();
        let paths = std::fs::read_dir(dir)?;

        for entry in paths {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "txt").unwrap_or(false) {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let content = std::fs::read_to_string(&path)?;
                let words = tokenize(&content);

                let mut seen: HashSet<String> = HashSet::new();
                for w in words {
                    if !seen.contains(&w) {
                        seen.insert(w.clone());
                        idx.add(w, name.clone());
                    }
                }
            }
        }

        Ok(idx)
    }

    /// 向索引中添加一个词-文档对
    fn add(&mut self, word: String, doc: String) {
        self.index
            .entry(word.to_lowercase())
            .or_default()
            .push(doc);
    }

    /// 搜索关键词，返回所有包含该词的文档名
    pub fn search(&self, keyword: &str) -> Vec<String> {
        let key = keyword.trim().to_lowercase();
        self.index.get(&key).cloned().unwrap_or_default()
    }

    /// 返回索引中的词条总数（词典大小）
    pub fn term_count(&self) -> usize {
        self.index.len()
    }

    /// 返回索引中所有词条
    pub fn terms(&self) -> Vec<&String> {
        let mut terms: Vec<&String> = self.index.keys().collect();
        terms.sort();
        terms
    }
}

/// 将文本按空白字符拆分为单词列表
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| {
            s.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|s| !s.is_empty())
        .collect()
}

/// 生成搜索结果 HTML 页面
pub fn to_html(keyword: &str, results: &[String]) -> String {
    let mut html = String::new();
    html.push_str("<html><head><meta charset=\"utf-8\"><title>MARS 搜索结果</title>");
    html.push_str("<style>body{font-family:sans-serif;margin:2em;background:#0a0a1a;color:#ddd;}");
    html.push_str("h1{color:#f90;} ul{list-style:none;padding:0;}");
    html.push_str("li{margin:0.5em 0;padding:0.8em;background:#1a1a2e;border-radius:4px;}");
    html.push_str("</style></head><body>");
    html.push_str(&format!(
        "<h1>MARS 小马尔斯 · 搜索结果</h1><p>搜索关键词: <strong>{}</strong></p>",
        keyword
    ));

    if results.is_empty() {
        html.push_str("<p>未找到相关文档。</p>");
    } else {
        html.push_str(&format!("<p>共找到 {} 个相关文档:</p><ul>", results.len()));
        for doc in results {
            html.push_str(&format!("<li>{}.txt</li>", doc));
        }
        html.push_str("</ul>");
    }

    html.push_str("<hr><p style=\"font-size:0.8em;color:#666;\">");
    html.push_str("MARS · 数据结构实验七 · 倒排索引搜索引擎</p></body></html>");
    html
}
