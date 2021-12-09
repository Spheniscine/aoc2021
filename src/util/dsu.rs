#[derive(Clone, Debug)]
pub struct Dsu {
    _id: Vec<usize>,
    _sz: Vec<usize>,
    num_components: usize
}

impl Dsu {
    pub fn new(n: usize) -> Dsu {
        Dsu { 
            _id: (0..n).collect(), 
            _sz: vec![1; n], 
            num_components: n
        }
    }

    pub fn root(&mut self, mut v: usize) -> usize {
        loop {
            let id = &mut self._id;
            let u = id[v];
            if u == v { return v; }
            let u = id[u];
            id[v] = u;
            v = u;
        }
    }

    pub fn join(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.root(u);
        let mut v = self.root(v);
        let id = &mut self._id;
        let sz = &mut self._sz;
        if u == v {
            return false;
        }
        if sz[u] < sz[v] {
            std::mem::swap(&mut u, &mut v);
        }
        id[v] = u;
        sz[u] += sz[v];
        self.num_components -= 1;
        true
    }

    pub fn is_joined(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn size(&mut self, v: usize) -> usize {
        let v = self.root(v);
        self._sz[v]
    }
}