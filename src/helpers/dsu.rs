use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect_vec(),
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let p = self.find(self.parent[x]);
        self.parent[x] = p;
        p
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parent[x] = y;
        }
    }
    pub fn into_inner(self) -> Vec<usize> {
        self.parent
    }
    pub fn counts(&mut self) -> Vec<usize> {
        let mut counts = vec![0; self.parent.len()];
        for i in 0..self.parent.len() {
            counts[self.find(i)] += 1;
        }

        counts
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 2);
        assert_eq!(uf.counts(), vec![0, 2, 0, 3, 0]);
    }
}
