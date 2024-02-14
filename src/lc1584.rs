
struct Edge {
    u: usize,
    v:usize,
    w: i32
}
struct UnionFind {
    fa: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            fa: (0..n).collect(),
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.find(self.fa[x]);
        }
        self.fa[x]
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (dx, dy) = (self.find(x), self.find(y));
        if dx != dy {
            self.fa[dx] = dy;
            return true;
        }
        return false;
    }
}
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut dist: Vec<Edge> = points.iter().enumerate()
            .flat_map(|(i, p1)| {
                points[i+1..].iter().enumerate().map(move |(j, p2)| {
                    let (dx, dy) = (p1[0] - p2[0], p1[1] - p2[1]);
                    let dis = dx.abs() + dy.abs();
                    Edge { u: i, v: i+j+1, w: dis}
                })
            }).collect();
        
        dist.sort_by_key(|a| a.w);
        let mut dsu = UnionFind::new(points.len());
        let mut res = 0;
        for e in dist {
            if dsu.union(e.u, e.v) {
                res += e.w;
            }
        }
        res
    }
}