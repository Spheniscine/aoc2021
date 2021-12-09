#[derive(Clone, Copy, Debug)]
pub struct GridHelper { n: usize, m: usize }
impl GridHelper {
    pub fn new(n: usize, m: usize) -> Self {
        Self { n, m }
    }
    pub fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        let mut p = 0;
        let mut res = [(0, 0); 4];
        if i > 0 { res[p] = (i-1, j); p += 1; }
        if i + 1 < self.n { res[p] = (i+1, j); p += 1; }
        if j > 0 { res[p] = (i, j-1); p += 1; }
        if j + 1 < self.m { res[p] = (i, j+1); p += 1; }
        (0..p).map(move |i| res[i])
    }
}