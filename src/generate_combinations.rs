pub fn generate_combinations(n: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut stack = vec![(0, Vec::new())];

    while let Some((current_sum, current)) = stack.pop() {
        if current_sum == n {
            result.push(current);
            continue;
        }
        for i in (1..n).rev() {
            if current_sum + i <= n {
                let mut next = current.clone();
                next.push(i);
                stack.push((current_sum + i, next));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_combinations() {
        assert_eq!(generate_combinations(2), vec![vec![1, 1]]);
        assert_eq!(
            generate_combinations(3),
            vec![vec![1, 1, 1], vec![1, 2], vec![2, 1]]
        );
        assert_eq!(
            generate_combinations(4),
            vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 2],
                vec![1, 2, 1],
                vec![1, 3],
                vec![2, 1, 1],
                vec![2, 2],
                vec![3, 1]
            ]
        );
        assert_eq!(
            generate_combinations(5),
            vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 2],
                vec![1, 1, 2, 1],
                vec![1, 1, 3],
                vec![1, 2, 1, 1],
                vec![1, 2, 2],
                vec![1, 3, 1],
                vec![1, 4],
                vec![2, 1, 1, 1],
                vec![2, 1, 2],
                vec![2, 2, 1],
                vec![2, 3],
                vec![3, 1, 1],
                vec![3, 2],
                vec![4, 1]
            ]
        );
    }
}
