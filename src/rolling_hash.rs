/// https://github.com/hatoo/competitive-rust-snippet
 
#[snippet = "RollingHash"]
#[allow(dead_code)]
/// Rolling hash algorithm
pub struct RollingHash {
    hash_pow_list: Vec<(u64, Vec<(u64, u64)>)>,
}

#[snippet = "RollingHash"]
#[allow(dead_code)]
impl RollingHash {
    pub fn new(s: &[u64]) -> RollingHash {
        RollingHash::with_base_mod_pairs(s, &[(1009, 1_000_000_007), (9973, 999_999_937)])
    }

    pub fn with_base_mod_pairs(s: &[u64], base_mod_pairs: &[(u64, u64)]) -> RollingHash {
        let hp_list = base_mod_pairs
            .iter()
            .map(|&(base, m)| {
                let mut hp = Vec::with_capacity(s.len() + 1);
                hp.push((0, 1));

                for (i, &x) in s.iter().enumerate() {
                    let (h, p) = hp[i];
                    hp.push(((h + x) * base % m, p * base % m));
                }
                (m, hp)
            })
            .collect();

        RollingHash {
            hash_pow_list: hp_list,
        }
    }

    // [l, r)
    pub fn get(&self, l: usize, r: usize) -> u64 {
        self.hash_pow_list
            .iter()
            .map(|&(m, ref hp)| (hp[r].0 + m - hp[l].0 * hp[r - l].1 % m) % m)
            .fold(0, |a, b| a ^ b)
    }

    pub fn len(&self) -> usize {
        self.hash_pow_list
            .first()
            .map(|v| v.1.len() - 1)
            .unwrap_or(0)
    }
}

#[test]
fn test_rolling_hash() {
    let seq: Vec<u64> = "abcabc".chars().map(|c| c as u64).collect();
    let rh = RollingHash::new(&seq);
    assert_eq!(rh.get(0, 3), rh.get(3, 6));
    assert_ne!(rh.get(0, 4), rh.get(3, 6));
    assert_ne!(rh.get(0, 3), rh.get(2, 5));
}

// https://qiita.com/keymoon/items/11fac5627672a6d6a9f6
use crate::xorshift::Xorshift;
#[snippet = "RoLiHa"]
struct RoLiHa {
    powMemo: Vec<u64>,
    hash: Vec<u64>,
}
#[snippet = "RoLiHa"]
const ROLIHA_MASK30: u64 = (1<<30) - 1;
#[snippet = "RoLiHa"]
const ROLIHA_MASK31: u64 = (1<<31) - 1;
#[snippet = "RoLiHa"]
const ROLIHA_MOD: u64 = (1<<61) - 1;
#[snippet = "RoLiHa"]
const ROLIHA_P: u64 = ROLIHA_MOD * ((1<<3) - 1);
#[snippet = "RoLiHa"]
impl RoLiHa {
    fn new(s: &[u64]) -> Self {
        let mut randgen = Xorshift::new();
        let rand = randgen.rand(std::i64::MAX as u64);
        let base = rand+129;

        let mut powMemo = vec![0; s.len()+1];
        powMemo[0]=1;
        for i in 1..powMemo.len() {
            powMemo[i] = Self::calcmod(Self::mul(powMemo[i-1], base));
        }

        let mut hash = vec![0; s.len()+1];
        for i in 0..s.len() {
            hash[i+1] = Self::calcmod(Self::mul(hash[i], base) + s[i]);
        }

        RoLiHa {
            powMemo: powMemo,
            hash: hash,
        }
    }

    // [l,r)
    pub fn get(&self, l: usize, r: usize) -> u64 {
        return Self::calcmod(self.hash[r] + ROLIHA_P - Self::mul(self.hash[l], self.powMemo[r-l]));
    }

    fn mul(l: u64, r: u64) -> u64 {
        let lu = l >> 31;
        let ld = l & ROLIHA_MASK31;
        let ru = r >> 31;
        let rd = r & ROLIHA_MASK31;
        let middle_bit = ld*ru + lu*rd;
        ((lu*ru)<<1) + ld*rd + ((middle_bit & ROLIHA_MASK30) << 31) + (middle_bit >> 30)
    }

    fn calcmod(x: u64) -> u64 {
        let mut x = (x & ROLIHA_MOD) + (x>>61);
        if x > ROLIHA_MOD {
            x -= ROLIHA_MOD;
        }
        x
    }
}

#[test]
fn test_roliha() {
    let seq: Vec<u64> = "abcabc".chars().map(|c| c as u64).collect();
    let rh = RoLiHa::new(&seq);
    assert_eq!(rh.get(0, 3), rh.get(3, 6));
    assert_ne!(rh.get(0, 4), rh.get(3, 6));
    assert_ne!(rh.get(0, 3), rh.get(2, 5));
}

fn mk_str(n: usize) -> Vec<u64> {
    let mut rng = Xorshift::new();
    let mut r = vec![];
    for _ in 0..n {
        // next()とかして値域フルにとると落ちます
        r.push(rng.rand(52));
    }
    r
}

fn mk_queries(n: usize) -> Vec<(usize,usize)> {
    let mut rng = Xorshift::new();
    let mut v = vec![];
    for _ in 0..n {
        let l = rng.rand(n as u64) as usize;
        let r = rng.rand(n as u64) as usize;
        let (x,y) = if l<=r {
            (l,r)
        } else {
            (r,l)
        };
        v.push((x,y))
    }
    v
}

#[bench]
fn bench_rolling_hash(b: &mut test::Bencher) {
    let s = mk_str(1_000_000);
    let qs = mk_queries(1_000_000);
    b.iter(move ||
        for i in 0..1 {
            let rh = RollingHash::new(&s);
            for (l,r) in &qs {
                rh.get(*l,*r);
            }
        }
    )
}

#[bench]
fn bench_roliha(b: &mut test::Bencher) {
    let s = mk_str(1_000_000);
    let qs = mk_queries(1_000_000);
    b.iter(move ||
        for i in 0..1 {
            let rh = RoLiHa::new(&s);
            for (l,r) in &qs {
                rh.get(*l,*r);
            }
        }
    )
}
