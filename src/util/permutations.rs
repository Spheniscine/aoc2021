#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Start, Looping, End
}

#[derive(Clone, Debug)]
pub struct Permutations {
    n: usize, r: usize, indices: Vec<usize>, cycles: Vec<usize>, state: State
}

impl Permutations {
    pub fn partial(n: usize, r: usize) -> Self {
        Self { n, r, indices: vec![], cycles: vec![], state: State::Start }
    }
    pub fn full(n: usize) -> Self { Self::partial(n, n) }

    pub fn next(&mut self) -> Option<&[usize]> {
        match self.state {
            State::Start => {
                if self.r > self.n { return None; }
                self.indices = (0..self.n).collect();
                self.cycles = (0..self.r).map(|i| self.n - i).collect();
                self.state = State::Looping;
                Some(&self.indices[0..self.r])
            },
            State::Looping => {
                loop {
                    for i in (0..self.r).rev() {
                        self.cycles[i] -= 1;
                        if self.cycles[i] == 0 {
                            let temp = self.indices[i];
                            for j in i..self.n-1 { self.indices[j] = self.indices[j+1]; }
                            self.indices[self.n-1] = temp;
                            self.cycles[i] = self.n - i;
                        } else {
                            let j = self.n - self.cycles[i];
                            self.indices.swap(i, j);
                            return Some(&self.indices[0..self.r])
                        }
                    }
                    self.state = State::End;
                    return None;
                }
            },
            State::End => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PermutationsOf<'a, T> { slice: &'a [T], p: Permutations }
pub trait PermutationSliceExt<'a, T> {
    fn permutations(&'a self) -> PermutationsOf<'a, T>;
    fn permutations_partial(&'a self, r: usize) -> PermutationsOf<'a, T>;
}
impl <'a, T: Clone> PermutationSliceExt<'a, T> for [T] {
    fn permutations(&'a self) -> PermutationsOf<'a, T> {
        PermutationsOf { slice: self, p: Permutations::full(self.len()) }
    }
    fn permutations_partial(&'a self, r: usize) -> PermutationsOf<'a, T> {
        PermutationsOf { slice: self, p: Permutations::partial(self.len(), r) }
    }
}
impl <'a, T: Clone> Iterator for PermutationsOf<'a, T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.p.next()?;
        let slice = self.slice;
        Some(p.iter().map(|&i| slice[i].clone()).collect())
    }
}