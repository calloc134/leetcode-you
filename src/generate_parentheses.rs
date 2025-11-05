use std::collections::HashSet;

pub struct Solution;
impl Solution {
    // (を1
    // )を0
    // として並べ方を考えていく

    // 左端は(なので1、右端は)なので0で固定
    // 全体長Nとすると、N-2個の要素を並べることを考える
    // 1がN/2-1個、0がN/2-1個

    // 単純に深さ優先探索で探索してみる
    // ただし、途中で)の数が(の数を上回る場合は探索を打ち切る
    // 両端は( = 1、) = 0で固定なので、最終的に1の数と0の数が等しくなることは保証される

    // 単に貪欲に1を優先して探索していく
    pub fn dfs_1than0(n: usize) -> HashSet<Vec<bool>> {
        let inner_card_length = n * 2 - 2; // (と ) で二倍、そこから両端を引く

        let mut results: HashSet<Vec<bool>> = HashSet::new();
        let mut stack: Vec<(Vec<bool>, usize, usize)> = Vec::new();
        stack.push((vec![true], 1, 0)); // (から始まるもの
        stack.push((vec![false], 0, 1)); // )から始まるもの

        while let Some((current, count_1, count_0)) = stack.pop() {
            // 終了条件
            if current.len() == inner_card_length {
                results.insert(current);
                continue;
            }

            // 1を追加できる場合
            if count_1 < n - 1 {
                let mut next = current.clone();
                next.push(true);
                stack.push((next, count_1 + 1, count_0));
            }

            // 0を追加できる場合
            if count_0 < count_1 {
                let mut next = current.clone();
                next.push(false);
                stack.push((next, count_1, count_0 + 1));
            }

            println!("stack detail: {:?}, results: {:?}", stack, results);
        }

        println!("final results before adding ends: {:?}", results);

        // 両端を付ける必要があるので付け加える

        let final_results: HashSet<Vec<bool>> = results
            .into_iter()
            .map(|mut v| {
                v.push(false); // 右端に)
                v.insert(0, true); // 左端に(
                v
            })
            .collect();
        final_results
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let results: HashSet<Vec<bool>> = Self::dfs_1than0(n as usize);
        let ans: Vec<String> = results
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|b| if b { '(' } else { ')' })
                    .collect::<String>()
            })
            .collect();
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
