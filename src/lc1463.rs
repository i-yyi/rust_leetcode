impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![0; m]; m]; n];
        dp[0][0][m-1] = grid[0][0] + grid[0][m-1];
        for i in 0..n-1 {
            for y1 in (0..m).take(i+1) {
                for y2 in (0..m).rev().take(i+1) {
                    for ty1 in y1.saturating_sub(1)..= y1 + 1 {
                        for ty2 in y2.saturating_sub(1)..= y2 + 1 {
                            if ty1 >= m || ty2 >= m {
                                continue;
                            }
                            println!("now cur : {} {} {}, to cur {} {}", i, y1, y2, ty1, ty2);
                            dp[i+1][ty1][ty2] = std::cmp::max(
                                dp[i+1][ty1][ty2],
                                dp[i][y1][y2] + 
                                if ty1 == ty2 {grid[i+1][y1]}
                                else {grid[i+1][y1] + grid[i+1][y2]}
                            )
                        }
                    } 
                }
            }
        }
        // println!(" {:?} ", dp);
        // sample path : 0 0 2, 1 0 1, 2 1 2, 3 0 2
        println!(" {} {} {} {}", dp[0][0][2], dp[1][0][1], dp[2][1][2], dp[3][0][2]);
        dp[n-1].iter()
        .flat_map(|row| row.into_iter().copied())
        .max()
        .unwrap()
    }
}