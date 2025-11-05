use std::collections::HashSet;

pub struct Solution;
impl Solution {
    // 内側（両端を除いた 2n-2 文字）を列挙し、最後に外側の '(' と ')' を付ける
    fn dfs_inner(n: usize) -> HashSet<Vec<bool>> {
        let inner_len = n * 2 - 2;

        let mut results: HashSet<Vec<bool>> = HashSet::new();
        let mut stack: Vec<(Vec<bool>, usize, usize)> = Vec::new();

        // 先頭の '(' はすでに置いたものとしてカウントする（open=1, close=0）
        stack.push((Vec::new(), 1, 0));

        while let Some((current, open, close)) = stack.pop() {
            if current.len() == inner_len {
                results.insert(current);
                continue;
            }

            // '(' を追加（総 open は最大 n まで）
            if open < n {
                let mut next = current.clone();
                next.push(true);
                stack.push((next, open + 1, close));
            }

            // ')' を追加（途中で閉じ超過しない）
            if close < open {
                let mut next = current.clone();
                next.push(false);
                stack.push((next, open, close + 1));
            }
        }

        results
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n.max(0) as usize;
        if n == 0 {
            return vec![String::new()];
        }

        let inner = Self::dfs_inner(n);

        let mut ans: Vec<String> = inner
            .into_iter()
            .map(|mut v| {
                // 右端に ')', 左端に '(' を付与
                v.push(false);
                v.insert(0, true);
                v.into_iter()
                    .map(|b| if b { '(' } else { ')' })
                    .collect::<String>()
            })
            .collect();

        // 期待順（辞書順）にそろえる
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parathesis_1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        )
    }

    #[test]
    fn test_generate_parathesis_2() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"])
    }

    #[test]
    fn test_generate_parathesis_3() {
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"])
    }
}
