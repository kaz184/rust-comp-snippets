fn bipartite_matching(g_list: &[Vec<usize>]) -> Vec<(usize,usize)> {

    fn dfs(v: usize, g_list: &[Vec<usize>], used: &mut [bool], matching: &mut [Option<usize>]) -> bool {
        used[v] = true;
        for i in 0..g_list[v].len() {
            let u = g_list[v][i];
            let w = matching[u];
            if w.is_none() || (!used[w.unwrap()] && dfs(w.unwrap(), g_list, used, matching)) {
                matching[v] = Some(u);
                matching[u] = Some(v);
                return true
            }
        }
        false
    }

    let n = g_list.len();
    let mut matching = vec![None; n];
    for v in 0..n {
        if matching[v].is_none() {
            let mut used = vec![false; n];
            dfs(v, g_list, &mut used, &mut matching);
        }
    }
    let mut res = vec![];
    for u in 0..matching.len() {
        let v0 = matching[u];
        if v0.is_some() {
            let v = v0.unwrap();
            assert!(u != v);
            if u < v {
                res.push((u,v));
            }
        }
    }
    res
}

struct BipartiteMatching {
    g: Vec<Vec<usize>>,
}
impl BipartiteMatching {
    fn new(n: usize) -> BipartiteMatching {
        BipartiteMatching {
            g: vec![vec![]; n],
        }
    }
    fn connect(&mut self, u: usize, v: usize) {
        assert!(u != v);
        self.g[u].push(v);
        self.g[v].push(u);
    }
    fn run(&self) -> Vec<(usize, usize)> {
        bipartite_matching(&self.g)
    }
}

#[test]
fn test_bipartite_matching() {
    let mut bpm = BipartiteMatching::new(4);
    bpm.connect(0,2);
    bpm.connect(0,3);
    bpm.connect(1,2);
    dbg!(bpm.run());
}