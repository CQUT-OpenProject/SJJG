pub mod region;
pub mod stack;

use crate::region::{N, adjacency};

/// 检查颜色 color[idx] = c 是否与所有已着色的邻居冲突
fn is_safe(map: &[[u8; N]; N], color: &[u8; N], idx: usize, c: u8) -> bool {
    for j in 0..N {
        if map[idx][j] == 1 && color[j] != 0 && color[j] == c {
            return false;
        }
    }
    true
}

/// 递归回溯求解 n-着色问题
/// 若存在合法 n-着色，将方案写入 out 并返回 true；否则返回 false
pub fn color_recursive(map: &[[u8; N]; N], n_colors: u8) -> Option<[u8; N]> {
    let mut color = [0u8; N];

    if try_color(map, &mut color, 0, n_colors) {
        Some(color)
    } else {
        None
    }
}

fn try_color(map: &[[u8; N]; N], color: &mut [u8; N], idx: usize, n_colors: u8) -> bool {
    if idx == N {
        return true;
    }

    for c in 1..=n_colors {
        if is_safe(map, color, idx, c) {
            color[idx] = c;
            if try_color(map, color, idx + 1, n_colors) {
                return true;
            }
            color[idx] = 0;
        }
    }

    false
}

/// 非递归回溯：借助顺序栈保存当前正在处理的区域下标
/// 栈中每个位置保存 (region_idx, next_color_to_try)
/// 算法思路：
///   1. 初始 color 全为 0
///   2. 若所有区域都已着色，结束
///   3. 取当前最前面未着色的区域 idx，从 1 号色开始试
///   4. 若有可用颜色，写入 color[idx]，将 (idx, 1) 入栈，继续处理下一个
///   5. 若 1..=n_colors 都不行，则回溯：弹出栈顶，取出其 next_color 继续尝试
///   6. 若回溯到第 0 个区域仍无解，退出失败
pub fn color_non_recursive(map: &[[u8; N]; N], n_colors: u8) -> Option<[u8; N]> {
    let mut color = [0u8; N];
    let mut st = stack::Stack::new(N + 1);

    // 第一个区域先尝试着色
    let mut idx: usize = 0;
    loop {
        // 找一个尚未着色的区域
        while idx < N && color[idx] != 0 {
            idx += 1;
        }

        if idx == N {
            return Some(color);
        }

        // 从当前 color[idx]+1 开始尝试（color[idx] 初值 0，所以从 1 开始）
        let mut c = color[idx] + 1;
        let mut found = false;

        while c <= n_colors {
            if is_safe(map, &color, idx, c) {
                found = true;
                break;
            }
            c += 1;
        }

        if found {
            // 写入新色并入栈
            color[idx] = c;
            st.push(idx);
            idx += 1;
        } else {
            // 当前区域无可用色：回溯
            color[idx] = 0;

            let mut backtrack_ok = false;
            while let Some(prev) = st.pop() {
                // 撤销前一个区域的着色，从其 next_color 继续
                let prev_color = color[prev];
                color[prev] = 0;
                let mut nc = prev_color + 1;
                let mut ok = false;
                while nc <= n_colors {
                    if is_safe(map, &color, prev, nc) {
                        ok = true;
                        break;
                    }
                    nc += 1;
                }

                if ok {
                    color[prev] = nc;
                    st.push(prev);
                    idx = prev + 1;
                    backtrack_ok = true;
                    break;
                }
                // 该区域所有颜色试完，继续弹出上一个
            }

            if !backtrack_ok {
                return None;
            }
        }
    }
}

/// 验证一个 n-着色方案是否合法
pub fn is_valid_coloring(map: &[[u8; N]; N], color: &[u8; N]) -> bool {
    for i in 0..N {
        for j in (i + 1)..N {
            if map[i][j] == 1 && color[i] == color[j] {
                return false;
            }
        }
    }
    true
}

/// 取默认邻接矩阵
pub fn default_map() -> [[u8; N]; N] {
    adjacency()
}

pub use region::region_name;
