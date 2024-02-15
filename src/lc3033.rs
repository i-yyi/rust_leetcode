use super::Solution;
impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == -1 {
                    matrix[i][j] = matrix.iter()
                    .map(|f| f[j])
                    .max()
                    .unwrap();
                }
            }
        }
        matrix
    }
}