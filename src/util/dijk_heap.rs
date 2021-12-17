#[derive(Debug, Clone)]
pub struct DijkHeap<Cost> {
    h: Vec<usize>,
    pos: Vec<usize>,
    cost: Vec<Option<Cost>>
}

impl <Cost: PartialOrd> DijkHeap<Cost> {
    pub fn new(n: usize) -> Self {
        Self { h: Vec::with_capacity(n), pos: vec![!0; n], cost: (0..n).map(|_| None).collect() }
    }

    fn _swap(&mut self, i: usize, j: usize) {
        self.h.swap(i, j);
        self.pos[self.h[i]] = i; self.pos[self.h[j]] = j;
    }

    pub fn promote(&mut self, key: usize, new_cost: Cost) -> bool {
        if self.cost_opt(key).map(|x| new_cost >= *x).unwrap_or(false) { return false; }
        let mut i = self.pos[key];
        if i == !0 {
            i = self.h.len();
            self.h.push(key);
            self.pos[key] = i;
        }

        while i > 0 {
            let par = i-1 >> 1;
            if new_cost >= *self.cost(self.h[par]) { break; }
            self._swap(i, par);
            i = par;
        }

        self.cost[key] = Some(new_cost);
        true
    }

    pub fn peek(&self) -> Option<usize> { self.h.first().copied() }

    pub fn pop(&mut self) -> Option<usize> {
        let hd = self.peek()?;
        self._swap(0, self.h.len() - 1);
        self.pos[hd] = !0;
        self.h.pop();

        let mut i = 0;
        loop {
            let l = i*2 + 1;
            if l >= self.h.len() { break; }
            let r = l + 1;

            let mut best = l;
            let mut p = self.cost(self.h[l]);
            if r < self.h.len() {
                let pr = self.cost(self.h[r]);
                if *pr < *p {
                    best = r;
                    p = pr;
                }
            }

            if *self.cost(self.h[i]) <= *p { break; }
            self._swap(i, best);
            i = best;
        }

        Some(hd)
    }

    pub fn cost(&self, key: usize) -> &Cost { self.cost[key].as_ref().unwrap() }
    pub fn cost_opt(&self, key: usize) -> Option<&Cost> { self.cost[key].as_ref() }
    pub fn into_costs(self) -> Vec<Option<Cost>> { self.cost }
}