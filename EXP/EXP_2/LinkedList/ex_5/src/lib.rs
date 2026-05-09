use std::collections::BTreeMap;

const EPS: f32 = 1e-6;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Term {
    exp: i32,
    coef: f32,
}

#[derive(Debug, Clone)]
struct Node {
    term: Term,
    next: Option<Box<Node>>,
}

#[derive(Debug, Clone, Default)]
pub struct Polynomial {
    head: Option<Box<Node>>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_terms(terms: &[(i32, f32)]) -> Self {
        let mut map = BTreeMap::new();
        for &(exp, coef) in terms {
            *map.entry(exp).or_insert(0.0) += coef;
        }

        let mut polynomial = Self::new();
        for (&exp, &coef) in map.iter() {
            if coef.abs() > EPS {
                polynomial.head = Some(Box::new(Node {
                    term: Term { exp, coef },
                    next: polynomial.head.take(),
                }));
            }
        }

        polynomial
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut terms = self.to_vec();
        terms.extend(other.to_vec());
        Self::from_terms(&terms)
    }

    pub fn to_vec(&self) -> Vec<(i32, f32)> {
        let mut result = Vec::new();
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            result.push((node.term.exp, node.term.coef));
            current = node.next.as_ref();
        }

        result
    }

    pub fn format(&self) -> String {
        let terms = self.to_vec();
        if terms.is_empty() {
            return "0".to_string();
        }

        let mut output = String::new();
        for (index, (exp, coef)) in terms.into_iter().enumerate() {
            let abs_coef = coef.abs();
            let body = match exp {
                0 => format_number(abs_coef),
                1 => {
                    if (abs_coef - 1.0).abs() <= EPS {
                        "x".to_string()
                    } else {
                        format!("{}x", format_number(abs_coef))
                    }
                }
                _ => {
                    if (abs_coef - 1.0).abs() <= EPS {
                        format!("x^{}", exp)
                    } else {
                        format!("{}x^{}", format_number(abs_coef), exp)
                    }
                }
            };

            if index == 0 {
                if coef < 0.0 {
                    output.push('-');
                }
                output.push_str(&body);
            } else if coef < 0.0 {
                output.push_str(" - ");
                output.push_str(&body);
            } else {
                output.push_str(" + ");
                output.push_str(&body);
            }
        }

        output
    }
}

fn format_number(value: f32) -> String {
    let mut text = format!("{:.6}", value);
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}
