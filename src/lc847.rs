use super::Solution;
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut dp = vec![vec![0x3f3f3f3f; 1 << n]; n];
        let mut queue = std::collections::VecDeque::new();

        for i in 0..n {
            dp[i][1 << i] = 0;
            queue.push_back((i, 1 << i));
        }

        while let Some((u, vis)) = queue.pop_front() {
            for &to in &graph[u] {
                let to = to as usize;
                let vis_to = vis | (1 << to);

                if dp[to][vis_to] > dp[u][vis] + 1 {
                    dp[to][vis_to] = dp[u][vis] + 1;
                    queue.push_back((to, vis_to));
                }
            }
        }

        dp.iter()
        .map(|d| d[(1 << n) - 1])
        .min()
        .unwrap()
    }
}