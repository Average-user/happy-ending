use std::collections::BTreeMap;
use itertools::Itertools;
use std::time::Instant;
use rayon::prelude::*;

type Crel = (i8,i8,usize,usize,usize,usize);
type Quad = (usize,usize,usize,usize);
type QuadV = (i8,i8,i8,i8);
type CompatibleRels = BTreeMap<Vec<i8>, Vec<Vec<i8>>>;

const N: usize = 17;
// valid signatures for a quadrilateral (see (2.3)). The ones that include 0's
// are the ones from which a valid signature in (2.3) can be reached.
const VALID: [QuadV;65] = [(1,1,1,1),(0,1,1,1),(1,0,1,1),(1,1,0,1),(1,1,1,0),(0,0,1,1),(0,1,0,1),(0,1,1,0),(1,0,0,1),(1,0,1,0),(1,1,0,0),(1,0,0,0),(0,1,0,0),(0,0,1,0),(0,0,0,1),(0,0,0,0),(1,1,1,-1),(0,1,1,-1),(1,0,1,-1),(1,1,0,-1),(0,0,1,-1),(0,1,0,-1),(1,0,0,-1),(0,0,0,-1),(1,1,-1,-1),(0,1,-1,-1),(1,0,-1,-1),(1,1,-1,0),(0,0,-1,-1),(0,1,-1,0),(1,0,-1,0),(0,0,-1,0),(1,-1,-1,-1),(0,-1,-1,-1),(1,-1,0,-1),(1,-1,-1,0),(0,-1,0,-1),(0,-1,-1,0),(1,-1,0,0),(0,-1,0,0),(-1,-1,-1,-1),(-1,0,-1,-1),(-1,-1,0,-1),(-1,-1,-1,0),(-1,0,0,-1),(-1,0,-1,0),(-1,-1,0,0),(-1,0,0,0),(-1,-1,-1,1),(0,-1,-1,1),(-1,0,-1,1),(-1,-1,0,1),(0,0,-1,1),(0,-1,0,1),(-1,0,0,1),(-1,-1,1,1),(0,-1,1,1),(-1,0,1,1),(-1,-1,1,0),(0,-1,1,0),(-1,0,1,0),(-1,1,1,1),(-1,1,0,1),(-1,1,1,0),(-1,1,0,0)];

// forced quadrilateral signatures. For example the ony valid signature
// that extends (-1,0,0,-1) is (-1,-1,-1,-1).
fn fillquad (xs: QuadV) -> Option<QuadV> {
    match xs {
        (-1,0,0,-1)  => Some((-1,-1,-1,-1)),
        (0,0,-1,1)   => Some((-1,-1,-1,1)),
        (0,0,1,-1)   => Some((1,1,1,-1)),
        (0,-1,1,0)   => Some((-1,-1,1,1)),
        (0,1,-1,0)   => Some((1,1,-1,-1)),
        (-1,1,0,0)   => Some((-1,1,1,1)),
        (1,-1,0,0)   => Some((1,-1,-1,-1)),
        (1,0,0,1)    => Some((1,1,1,1)),
        (-1,0,-1,-1) => Some((-1,-1,-1,-1)),
        (-1,-1,0,-1) => Some((-1,-1,-1,-1)),
        (0,-1,-1,1)  => Some((-1,-1,-1,1)),
        (-1,0,-1,1)  => Some((-1,-1,-1,1)),
        (-1,-1,1,0)  => Some((-1,-1,1,1)),
        (0,-1,1,1)   => Some((-1,-1,1,1)),
        (0,1,-1,-1)  => Some((1,1,-1,-1)),
        (-1,1,0,1)   => Some((-1,1,1,1)),
        (0,1,1,-1)   => Some((1,1,1,-1)),
        (-1,1,1,0)   => Some((-1,1,1,1)),
        (1,-1,0,-1)  => Some((1,-1,-1,-1)),
        (1,-1,-1,0)  => Some((1,-1,-1,-1)),
        (1,0,1,-1)   => Some((1,1,1,-1)),
        (1,0,1,1)    => Some((1,1,1,1)),
        (1,1,-1,0)   => Some((1,1,-1,-1)),
        (1,1,0,1)    => Some((1,1,1,1)),
        _            => None
    }
}

// Practical bijection between {0,1,..,n(n-1)(n-2)/6 -1} and {(a,b,c) : 1 <= a < b < c <= n} forall n.
fn cd(a: usize, b: usize, c: usize) -> usize {
    return ((c-1)*(c-2)*(c-3))/6 + ((b-1)*(b-2))/2 + a -1;
}

// convex relations for an hexagon (see (4.1)).
fn relations(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> Vec<Crel> {
    return vec![( 1, 1,cd(a,b,c),cd(b,c,d),cd(c,d,e),cd(d,e,f)),
                ( 1,-1,cd(a,b,c),cd(b,c,d),cd(c,d,f),cd(a,e,f)),
                ( 1,-1,cd(a,b,c),cd(b,c,e),cd(c,e,f),cd(a,d,f)),
                ( 1,-1,cd(a,b,d),cd(b,d,e),cd(d,e,f),cd(a,c,f)),
                ( 1,-1,cd(a,c,d),cd(c,d,e),cd(d,e,f),cd(a,b,f)),
                (-1,-1,cd(a,b,c),cd(b,c,f),cd(a,d,e),cd(d,e,f)),
                (-1,-1,cd(a,b,d),cd(b,d,f),cd(a,c,e),cd(c,e,f)),
                (-1,-1,cd(a,c,d),cd(c,d,f),cd(a,b,e),cd(b,e,f))];
}

// Returns a vector v such that v[cd(a,b,c)] has all the convex relations involving (a,b,c).
fn convex_rels6() -> Vec<Vec<Crel>> {
    let mut convexr: Vec<Vec<Crel>> = vec![vec![];N*(N-1)*(N-2)/6];
    for (a,b,c,d) in (1..=N).tuple_combinations::<(_,_,_,_)>() {
        for (e,f) in (d+1..=N) .tuple_combinations::<(_,_)>() {
            for (x,y,r1,r2,r3,r4) in relations(a,b,c,d,e,f) {
                for i in vec![r1,r2,r3,r4].iter() {
                    convexr[*i].push((x,y,r1,r2,r3,r4));
                }
        }
        }
    }
    return convexr;
}

// All signatures for a hexagon which are concave and satisfy geometric conditions (Omega*).
fn concave6() -> Vec<Vec<i8>> {
    let mut vec: Vec<Vec<i8>> = Vec::new();
    'x: for i in 0..2_usize.pow(20) {
        let s = format!("{:020b}", i);
        let v: Vec<i8> = s.chars().map(|c| match c {'0' => -1, _ => 1}).collect();
        for (p,q,r1,r2,r3,r4) in relations(1,2,3,4,5,6) {
            let (x1,x2,x3,x4) = (v[r1],v[r2],v[r3],v[r4]);
            if x1 == x2 && x1 == p*x3 && x1 == q*x4 { continue 'x; }
        }
        for (a,b,c,d) in (1..=6).tuple_combinations::<(_,_,_,_)>() {
            let quad = (v[cd(a,b,c)],v[cd(a,b,d)],v[cd(a,c,d)],v[cd(b,c,d)]);
            if VALID.iter().all(|y| quad != *y) { continue 'x; }
        }
        vec.push(v)
    }
    return vec;
}

// By viewing each element in omega as a binary number (1's are 1's, -1's are 0's)
// one can identify each one of them by a number which we call its index.
fn assignment_idx(v: &Vec<i8>) -> i32 {
    return (0..v.len()).map(|i| match v[v.len()-i-1] {1 => 2_i32.pow(i as u32), _ => 0}).sum()
}

// Creates a Map which assigns for every v in Omega its compatible set of signatures.
// The ones that can be assigned to u_{j+1} if v is assigned to u_{j}. See (3.4) and
// the first note on efficiency at the end of page 8.
fn compatible(omega: & Vec<Vec<i8>>) -> CompatibleRels {
    let mut x = BTreeMap::new();
    for v in omega.iter() {
        let mut vec: Vec<Vec<i8>> = Vec::new();
        for u in omega.iter() {
            if (1..6).tuple_combinations::<(_,_,_)>().all(|(a,b,c)| v[cd(a+1,b+1,c+1)] == u[cd(a,b,c)]) {
                vec.push(u.to_vec());
            }
        }
        x.insert(v.to_vec(),vec);
    }
    return x;
}

// For each triple (a,b,c) quads[cd(a,b,c)] will store all quadrilateral
// relations that involve a b and c.
fn quadrilaterals() -> Vec<Vec<Quad>> {
    let mut quads: Vec<Vec<Quad>> = vec![vec![];N*(N-1)*(N-2)/6];
    for (a,b,c,d) in (1..=N).tuple_combinations::<(_,_,_,_)>() {
        let (x,y,z,w) = (cd(a,b,c),cd(a,b,d),cd(a,c,d),cd(b,c,d));
        quads[x].push((x,y,z,w));
        quads[y].push((x,y,z,w));
        quads[z].push((x,y,z,w));
        quads[w].push((x,y,z,w));
    }
    return quads;
}


// ================= END OF PRE-PROCESSING INTENDED FUNCTIONS ======================

// Given some assignment f, and a list of the recent assigned indexes, it checkes
// weather a contradiction has been reached. And recursively keeps assigning elements
// if they are forced to avoid satisfying a convex relation, or to keep the geometric
// conditions all satisfied. See Step 1 of the algorithm in the paper.
fn add(rels: &Vec<Vec<Crel>>, quads: & Vec<Vec<Quad>>, f: &mut Vec<i8>, hs: &mut Vec<usize>, added: &Vec<usize>) -> bool {
    if added.is_empty() { return true }
    let mut toadd: Vec<(usize,i8)> = Vec::new();
    for cod in added.iter() {
        for (p,q,r1,r2,r3,r4) in rels[*cod].iter() {
            let (x1,x2,x3,x4) = (f[*r1],f[*r2],f[*r3],f[*r4]);
            if x1 != 0 && x2 != 0 && x3 != 0 && x4 != 0 && (x1 == x2 && x1 == p*x3 && x1 == q*x4) {
                return false;
            }
            if x2 != 0 && x3 != 0 && x4 != 0 && x2 == p*x3 && x2 == q*x4 { toadd.push((*r1,-x2)); }
            if x1 != 0 && x3 != 0 && x4 != 0 && x1 == p*x3 && x1 == q*x4 { toadd.push((*r2,-x1)); }
            if x1 != 0 && x2 != 0 && x4 != 0 && x1 == x2 && x1 == q*x4 { toadd.push((*r3,-p*x1)); }
            if x1 != 0 && x2 != 0 && x3 != 0 && x1 == x2 && x1 == p*x3 { toadd.push((*r4,-q*x1)); }
        }
        for (x,y,z,w) in quads[*cod].iter() {
            let (a,b,c,d) = (f[*x],f[*y],f[*z],f[*w]);
            if VALID.iter().all(|y| (a,b,c,d) != *y) { return false; }
            match fillquad((a,b,c,d)) {
                None => continue,
                Some((x1,y1,z1,w1)) => {
                    if a == 0 { toadd.push((*x,x1)); }
                    if b == 0 { toadd.push((*y,y1)); }
                    if c == 0 { toadd.push((*z,z1)); }
                    if d == 0 { toadd.push((*w,w1)); }
                }
            }
        }
    }
    let mut toaddi = Vec::new();
    for (x,v) in toadd.iter() {
        let val = f[*x];
        if val == 0 {
            f[*x] = *v;
            hs.push(*x);
            toaddi.push(*x);
        }
        else if *v != val { return false; }
    }
    return add(rels, quads, f, hs, &toaddi);
}

// Sets the relations involved in u_j to a particular state given by v.
fn set_u(rels: &Vec<Vec<Crel>>, quads: & Vec<Vec<Quad>>,
            f: &mut Vec<i8>, hs: &mut Vec<usize>, j: usize, v: &Vec<i8>) -> bool {
    let mut toadd = Vec::new();
    for (a,b,c) in (1..=6).tuple_combinations::<(_,_,_)>() {
        let cod = cd(a+j-1,b+j-1,c+j-1);
        let val = f[cod];
        if val == 0 {
            toadd.push(cod);
            f[cod] = v[cd(a,b,c)];
            hs.push(cod);
        } else if val != v[cd(a,b,c)] { return false; }
    }
    return add(rels, quads, f, hs, &toadd);
}

// returns a vector of the state of relations involved in u_j.
fn get_u(f: &mut Vec<i8>, j: usize) -> Vec<i8> {
    let mut v = vec![0;20];
    for (a,b,c) in (1..=6).tuple_combinations::<(_,_,_)>() {
        v[cd(a,b,c)] = f[cd(a+j-1,b+j-1,c+j-1)];
    }
    return v;
}

// given an assignment f and the information of assigned elements in hs
// it restores f to when it had k elements assigned. This assumes that
// hs has been faithfully filled.
fn restore(f: &mut Vec<i8>, hs: &mut Vec<usize>, k: usize) -> () {
    for i in k..hs.len() { f[hs[i]] = 0; }
    for _ in 0..hs.len() - k { hs.pop(); }
}

// Performs the one-bit-check described in page 10.
fn one_bit_check(rels: &Vec<Vec<Crel>>, quads: & Vec<Vec<Quad>>,
                 f: &mut Vec<i8>, hs: &mut Vec<usize>) -> bool {
    let k = hs.len();
    for i in 0..N*(N-1)*(N-2)/6 {
        if f[i] == 0 {
            hs.push(i); f[i] = 1;
            let b1 = add(rels,quads,f,hs,&vec![i]);
            restore(f,hs,k);
            if b1 { continue; }
            hs.push(i); f[i] = -1;
            let b2 = add(rels,quads,f,hs,&vec![i]);
            restore(f,hs,k);
            if b2 { continue; }
            return false;
        }
    }
    return true;
}

// Performs the two-bit-check described in page 10.
fn two_bit_check(rels: &Vec<Vec<Crel>>, quads: & Vec<Vec<Quad>>,
                 f: &mut Vec<i8>, hs: &mut Vec<usize>) -> bool {
    let k = hs.len();
    for i in 0..N*(N-1)*(N-2)/6 {
        't: for j in i+1..N*(N-1)*(N-2)/6 {
            if f[i] == 0 && f[j] == 0 {
                for (fi,fj) in [(1_i8,1_i8), (1,-1), (-1,1), (-1,-1)].iter() {
                    hs.push(i); hs.push(j);
                    f[i] = *fi; f[j] = *fj;
                    let b = add(rels,quads,f,hs,&vec![i,j]);
                    restore(f,hs,k);
                    if b { continue 't; }
                }
                return false;
            }
        }
    }
    return true;
}

// It attempts to assign every triplet involved in some u_j without satisfying none
// of the convex relations, and all of the geometric ones.
// It performs other types of checks if it assigned all triplets involved in
// u_j for all j in 1..=12.
// This assumes triplets in u_1 are already assigned.
fn search(rels: &Vec<Vec<Crel>>, comp: & CompatibleRels, quads: & Vec<Vec<Quad>>,
          f: &mut Vec<i8>, hs: &mut Vec<usize>, count: &mut i64) -> () {
    match (1..=N-5).find(|j| get_u(f,*j).iter().any(|x| *x == 0)) {
        None => if one_bit_check(rels,quads,f,hs) && two_bit_check(rels,quads,f,hs)
                { *count = *count + 1 },
        Some(j) => {
            let com = comp.get(&get_u(f,j-1)).unwrap();
            let u = get_u(f, j);
            for v in com.iter() {
                if (0..v.len()).all(|i| u[i] == 0 || u[i] == v[i]) {
                    let k = hs.len();
                    let b = set_u(rels,quads,f,hs,j,v);
                    if b { search(rels,comp,quads,f,hs,count); }
                    restore(f,hs,k);
                }
            }
        }
    }
}

fn try_assignment(rels: &Vec<Vec<Crel>>, comp: & CompatibleRels, quads: & Vec<Vec<Quad>>, v: &Vec<i8>) {
    let mut count: i64 = 0;
    let mut f: Vec<i8> = vec![0;N*(N-1)*(N-2)/6];
    let mut history: Vec<usize> = Vec::new();
    let now = Instant::now();
    let b = set_u(rels,quads,&mut f,&mut history,1,v);
    if b { search(rels,comp,quads,&mut f,&mut history,&mut count); }
    let elapsed_time = now.elapsed();
    println!("idx: {}  n: {}  t: {}  v: {:?}",assignment_idx(v),count,elapsed_time.as_secs() as f32/60.0,*v);
}

pub fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    let omega = concave6();
    let comp = compatible(& omega);
    let convexr = convex_rels6();
    let quads = quadrilaterals();
    //let to_run: Vec<i32> = omega.iter().filter(|v| v[0] == 1).map(|v| assignment_idx(v)).collect();
    let to_run: Vec<i32> = vec![524288,524303,524343,524351,524415,524927,525183,525311,525375,525439,525951,527360,528383,531456,531472,531474,531510,531568,531570,531574,531582,531583,532094,532095,532350,532478,532479,564224,564240,564242,564274,564336,564338,564848,564850,564858,564863,565114,565246,565247,580608,581631,588800,588848,588912,589424,589680,589688,589808,589823,786432,786447,786463,786959,786975,787039,787295,787455,787456,787472,787474,787487,787999,788063,788095,820059,820063,820095,820223,820224,820240,820242,820754,820755,820763,820827,820831,820863,821083,821087,821119,821247,824320,824336,824338,824832,824848,824850,824914,824922,824923,825179,825183,825215,825343,826368,826384,826386,826898,840704,841216,841232,841296,841552,841563,841567,841599,841727,843280,843344,843376,843600,850944,851456,851472,851536,851792,851952,851967,918527,950784,950787,950795,950799,951051,967169,967425,967433,972288,972544,972608,974336,974592,974656,982016,982528,982784,982848,983040,983041,983047,983055,983567,983823,983887,983951,984063,1015808,1015809,1015811,1016320,1016321,1016323,1016331,1016587,1016651,1016719,1016831,1032192,1032193,1032704,1032705,1032961,1032969,1040384,1040385,1040897,1041153,1041161,1041225,1041281,1041289,1041353,1047552,1048064,1048320,1048384,1048448,1048512];
    omega.par_iter().for_each(|v| if to_run.iter().any(|i| assignment_idx(v) == *i)
                              {try_assignment(&convexr,&comp,&quads,v)});
}
