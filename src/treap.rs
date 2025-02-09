/// https://www.slideshare.net/iwiwi/2-12188757

#[snippet = "Treap"]
mod treap {
    #[derive(Clone, Debug)]
    pub struct Node {
        pub v: i64,
        pri: u64,
        lch: Option<Box<Node>>,
        rch: Option<Box<Node>>,
        cnt: usize,
        sum: i64,
    }

    pub fn new_node(v: i64, rand: u64) -> Node {
        Node {
            v: v,
            pri: rand,
            lch: None.into(),
            rch: None.into(),
            cnt: 1,
            sum: v
        }
    }

    pub fn count(t: &Option<Box<Node>>) -> usize {
        match *t {
            Some(ref x) => x.cnt,
            None => 0,
        }
    }

    pub fn sum(t: &Option<Box<Node>>) -> i64 {
        match *t {
            Some(ref x) => x.sum,
            None => 0,
        }
    }

    fn update(t: Box<Node>) -> Box<Node> {
        let mut t = t;
        t.cnt = count(&t.lch) + count(&t.rch) + 1;
        t.sum = sum(&t.lch) + sum(&t.rch) + t.v;
        t
    }

    pub fn merge(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
        if l.is_none() && r.is_none() {
            return None
        }
        if l.is_none() {
            return r
        }
        if r.is_none() {
            return l
        }

        assert!(l.is_some() && r.is_some());
        let mut l = l.unwrap();
        let mut r = r.unwrap();

        if l.pri > r.pri {
            let old_rch = l.rch.take();
            l.rch = merge(old_rch, Some(r).into());
            update(l).into()
        } else {
            let old_lch = r.lch.take();
            r.lch = merge(Some(l).into(), old_lch);
            update(r).into()
        }
    }

    // [0,k),[k,n]
    pub fn split(t: Option<Box<Node>>, k: usize) -> (Option<Box<Node>>, Option<Box<Node>>) {
        if t.is_none() {
            return (None.into(), None.into())
        }
        let mut t = t.unwrap();

        let lcnt = count(&t.lch);
        if k <= lcnt {
            let old_lch = t.lch.take();
            let s = split(old_lch, k);
            t.lch = s.1;
            (s.0, Some(update(t)).into())
        } else {
            let old_rch = t.rch.take();
            let s = split(old_rch, k - lcnt - 1);
            t.rch = s.0;
            (Some(update(t)).into(), s.1)
        }
    }

    // split is fast but merge is slow
    pub fn insert(t: Box<Node>, k: usize, v: i64, rand: u64) -> Option<Box<Node>> {
        let (l,r) = split(Some(t).into(), k);
        let newt = merge(l, Some(new_node(v, rand).into()));
        let newt = merge(newt, r);
        newt
    }

    pub fn erase(t: Box<Node>, k: usize) -> Option<Box<Node>> {
        // [0,k),[k,k+1)[k+1,n)
        let (t1, rest) = split(Some(t).into(), k);
        let (t2, t3) = split(rest, 1);
        merge(t1, t3)
    }

    pub fn bisect(t: &Option<Box<Node>>, v: i64) -> usize {
        match *t {
            None => 0,
            Some(ref n) => {
                if v <= n.v {
                    bisect(&n.lch, v)
                } else {
                    count(&n.lch) + 1 + bisect(&n.rch, v)
                }
            }
        }
    }
}

use crate::xorshift::Xorshift;

#[snippet = "Treap"]
struct Treap {
    rng: Xorshift,
    t: Option<Box<treap::Node>>,
}
#[snippet = "Treap"]
impl Treap {
    pub fn new() -> Treap {
        Treap {
            rng: Xorshift::new(),
            t: None,
        }
    }
    #[doc = "[l,r)"]
    pub fn lower_bound(&mut self, l: usize, r: usize, v: i64) -> usize {
        let t = self.t.take();
        let (lt, t) = treap::split(t, l);
        let (t, rt) = treap::split(t, r-l);
        let idx = treap::count(&lt) + treap::bisect(&t, v);
        self.t = treap::merge(lt, treap::merge(t, rt));
        idx
    }
    pub fn orderd_insert(&mut self, v: i64) {
        if self.t.is_none() {
            self.insert(0, v);
        } else {
            let n = self.len();
            let ins_pos = self.lower_bound(0, n, v);
            self.insert(ins_pos, v);
        }
    }
    pub fn insert(&mut self, k: usize, v: i64) {
        if self.t.is_none() {
            self.t = Some(treap::new_node(v, self.rng.next()).into());
        } else {
            let t = self.t.take().unwrap();
            self.t = treap::insert(t, k, v, self.rng.next());
        }
    }
    pub fn erase(&mut self, k: usize) {
        if self.t.is_some() {
            let t = self.t.take().unwrap();
            self.t = treap::erase(t, k).into();
        }
    }
    #[doc = "split into [l,r)+[r,n)"]
    pub fn split(self, k: usize) -> (Treap, Treap) {
        let (a, b) = treap::split(self.t, k);
        (Treap { rng: self.rng.clone(), t: a }, Treap { rng: self.rng.clone(), t: b })
    }
    pub fn len(&self) -> usize {
        treap::count(&self.t)
    }
    #[doc = "[l,r)"]
    pub fn sum(&mut self, l: usize, r: usize) -> i64 {
        if self.t.is_none() {
            return 0
        } else {
            let t = self.t.take();
            // split into a1,b1,b2
            let (a1, a2) = treap::split(t, l);
            let (b1, b2) = treap::split(a2, r-l);
            let res = treap::sum(&b1);
            self.t = treap::merge(treap::merge(a1, b1), b2);
            res
        }
    }
    pub fn get(&mut self, k: usize) -> i64 {
        self.sum(k, k+1)
    }
}

#[test]
fn test_treap_lower_bound() {
    let mut rng = Xorshift::new();
    let mut tr = Treap::new();
    let mut v = vec![];
    let n = 10000;
    for i in 0..n {
        let x = rng.rand(1000) as i64;

        v.push(x);
        v.sort();
        let mut k1 = i;
        for k in 0..i {
            if v[k]==x {
                k1 = k;
                break;
            }
        }

        tr.orderd_insert(x);
        let k2 = tr.lower_bound(0,i,x);

        assert_eq!(k2,k1);
    }
}

#[test]
fn test_treap() {
    let mut tr = Treap::new();
    assert_eq!(tr.len(), 0);
    tr.insert(0, 3);
    assert_eq!(tr.len(), 1);
    tr.insert(0, 5);
    assert_eq!(tr.len(), 2);
    assert_eq!(tr.sum(0, 1), 5);
    assert_eq!(tr.sum(1, 2), 3);
    tr.insert(0, 1);
    assert_eq!(tr.sum(0, 1), 1);
    assert_eq!(tr.sum(1, 2), 5);
    assert_eq!(tr.sum(2, 3), 3);
    tr.erase(1);
    assert_eq!(tr.sum(0, 1), 1);
    assert_eq!(tr.sum(1, 2), 3);
}

#[test]
fn test_treap_ref() {
    let mut rng = Xorshift::new();
    let mut v = vec![];
    let mut tr = Treap::new();
    let n = 100000;
    for _ in 0..n {
        let x = rng.rand(1000) as i64;
        v.push(x);
        tr.orderd_insert(x);
    }
    v.sort();

    for i in 0..n {
        assert_eq!(tr.get(i), v[i]);
    }
}