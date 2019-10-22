use std::collections::VecDeque;

struct HLDecomposition {
    n: usize,
    g: Vec<Vec<usize>>,
    subcnt: Vec<usize>,
    depth: Vec<usize>,
    par: Vec<Option<usize>>,
    heavy_next: Vec<Option<usize>>,
    heavy_head: Vec<usize>,
    real_to_virt: Vec<usize>,
    virt_to_real: Vec<usize>,
}

impl HLDecomposition {

    fn new(n: usize) -> Self {
        HLDecomposition {
            n: n,
            g: vec![vec![]; n],
            subcnt: vec![0; n],
            depth: vec![0; n],
            par: vec![None; n],
            heavy_next: vec![None; n],
            heavy_head: vec![n; n],
            real_to_virt: vec![n; n],
            virt_to_real: vec![n; n],
        }
    }

    fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }

    fn build(&mut self, root: usize) {
        self.dfs1(root);
        self.dfs2(root);
        self.bfs(root);
    }

    // 部分木の大きさを計算する
    fn dfs1(&mut self, root: usize) {
        self.depth[root] = 0;
        self.par[root] = None;
        self.dfs1_sub(root, None);
    }
    fn dfs1_sub(&mut self, u: usize, par: Option<usize>) -> usize {
        let mut cnt = 1;
        for v in self.g[u].clone() {
            if Some(v) == par { continue; }
            self.depth[v] = self.depth[u] + 1;
            self.par[v] = Some(u);
            cnt += self.dfs1_sub(v, Some(u));
        }
        self.subcnt[u] = cnt;
        cnt
    }
    
    // ヘビーパスを探す
    fn dfs2(&mut self, root: usize) {
        self.dfs2_sub(root, None);
    }
    fn dfs2_sub(&mut self, u: usize, par: Option<usize>) {
        let mut maxv = 0;
        let mut heavy_next = None;

        let cld = self.g[u].clone();

        // ヘビーパスを決める
        for &v in &cld {
            if Some(v) == par { continue; }
            if self.subcnt[v] > maxv { 
                maxv = self.subcnt[v];
                heavy_next = Some(v);
            }
        }
        // ヘビーパスがあるならそれを伸ばす
        if let Some(hn) = heavy_next {
            self.heavy_next[u] = Some(hn);
            self.dfs2_sub(hn, Some(u));
        }
        // ライトパスはまたルートからやり直し
        for &v in &cld {
            if Some(v) == par || Some(v) == heavy_next { continue; }
            self.dfs2_sub(v, Some(u));
        }
    }

    fn bfs(&mut self, root: usize) {
        let mut cur_virt_id = 0;
        let mut q = VecDeque::new();
        q.push_back(root);
        // ヘビーパスの先頭から下っていく
        while let Some(h) = q.pop_front() {
            let mut cur0 = Some(h);
            while cur0.is_some() {
                let cur = cur0.unwrap();
                self.real_to_virt[cur] = cur_virt_id;
                self.virt_to_real[cur_virt_id] = cur;
                cur_virt_id += 1;
                self.heavy_head[cur] = h;
                for v in self.g[cur].clone() {
                    if Some(v) == self.par[cur] || Some(v) == self.heavy_next[cur] { continue; }
                    q.push_back(v);
                }
                cur0 = self.heavy_next[cur];
            }
        }
    }
}

#[test]
fn test_hl_decomposition() {
    let mut hl = HLDecomposition::new(13);
    let es = vec![(0,1),(0,2),(0,3),(1,4),(1,5),(4,8),(4,9),(2,6),(6,10),(6,11),(6,12),(3,7)];
    for (u,v) in es {
        hl.connect(u,v);
    }
    hl.build(0);
    // dbg!(&hl.depth);
    // dbg!(&hl.par);
    // dbg!(&hl.subcnt);
    // dbg!(&hl.heavy_next);
    dbg!(&hl.real_to_virt);
    dbg!(&hl.heavy_head);
}