use indexmap::IndexMap;

use crate::aoc_base::Day;

pub struct Day12;

#[derive(Debug, Clone)]
pub struct Graph {
    map: IndexMap<String, Vec<usize>>
}

impl Graph {
    fn register(&mut self, name: &str) -> usize {
        let e = self.map.entry(name.to_owned());
        let idx = e.index();
        e.or_default();
        idx
    }

    fn is_big(&self, index: usize) -> bool {
        self.map.get_index(index).unwrap().0.as_bytes()[0].is_ascii_uppercase()
    }

    fn is_small(&self, index: usize) -> bool {
        !self.is_big(index)
    }

    fn num(&self) -> usize {
        self.map.len()
    }
}

impl Day for Day12 {
    type Parsed = Graph;

    type Output1 = i32;

    type Output2 = i32;

    fn num() -> usize {
        12
    }

    fn title() -> &'static str {
        "Passage Pathing"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut res = Graph { map: IndexMap::new() };

        for ln in input.lines() {
            let mut ln = ln.split('-');
            let u = res.register(ln.next().unwrap());
            let v = res.register(ln.next().unwrap());
            res.map[u].push(v);
            res.map[v].push(u);
        }

        res
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let n = data.num();
        let start = data.map.get_index_of("start").unwrap();
        let end = data.map.get_index_of("end").unwrap();

        let mut vis = vec![false; n];
        vis[start] = true;
        let mut stk = vec![(start, 0usize)];
        let mut ans = 0;

        'dfs: while let Some((u, mut i)) = stk.pop() {
            while i < data.map[u].len() {
                let v = data.map[u][i];
                i += 1;
                if v == end { ans += 1; }
                else if !vis[v] {
                    if data.is_small(v) { vis[v] = true; }
                    stk.push((u, i));
                    stk.push((v, 0));
                    continue 'dfs;
                }
            }

            vis[u] = false;
        }

        ans
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let n = data.num();
        let start = data.map.get_index_of("start").unwrap();
        let end = data.map.get_index_of("end").unwrap();

        let mut vis = vec![0; n];
        vis[start] = 2;
        let mut stk = vec![(start, 0usize, false)];
        let mut ans = 0;

        'dfs: while let Some((u, mut i, pow)) = stk.pop() {
            while i < data.map[u].len() {
                let v = data.map[u][i];
                i += 1;
                if v == end { 
                    // eprintln!("{:?}", stk.iter().map(|x| &data.names[x.0]).collect::<Vec<_>>());
                    ans += 1; 
                }
                else {
                    let mut npow = pow;
                    if data.is_small(v) {
                        if vis[v] == 2 { continue; }
                        if vis[v] == 1 {
                            if pow { continue; }
                            npow = true;
                        }
                    }
                    vis[v] += 1;
                    stk.push((u, i, pow));
                    stk.push((v, 0, npow));
                    continue 'dfs;
                }
            }

            vis[u] -= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_input() {
        let inp = indoc!{"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "};
        Day12::test(InputSource::Literal(inp), Some(10), Some(36));
    }

    #[test]
    fn gmail() {
        Day12::test(InputSource::gmail(12), Some(4411), Some(136767))
    }
}