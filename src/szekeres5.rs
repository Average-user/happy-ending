use std::collections::BTreeMap;
use itertools::Itertools;

// Algorithm of "Computer solution to the 17-point Erdős-Szekeres problem".

type Crel = (i8,usize,usize,usize);
type CompatibleRels = BTreeMap<Vec<i8>, Vec<Vec<i8>>>;

fn cd(a: usize, b: usize, c: usize) -> usize {
    return ((c-1)*(c-2)*(c-3))/6 + ((b-1)*(b-2))/2 + a -1;
}

fn convex_rels5p(a: usize, b: usize, c: usize, d: usize, e: usize) -> Vec<Crel> {
    return vec![(1, cd(a,b,c),cd(b,c,d),cd(c,d,e)), (-1,cd(a,b,c),cd(b,c,e),cd(a,d,e)),
                (-1,cd(a,b,d),cd(b,d,e),cd(a,c,e)), (-1,cd(a,c,d),cd(c,d,e),cd(a,b,e))];
}

fn convex_rels5(n: usize) -> Vec<Vec<Crel>> {
    let mut convexr: Vec<Vec<Crel>> = vec![vec![];n*(n-1)*(n-2)/6];
    for (a,b,c,d) in (1..n+1).tuple_combinations::<(_,_,_,_)>() {
        for e in d+1..n+1 {
            for (c,r1,r2,r3) in convex_rels5p(a,b,c,d,e) {
                for i in vec![r1,r2,r3].iter() {
                    convexr[*i].push((c,r1,r2,r3));
                }
            }
        }
    }
    return convexr;
}

fn concave5() -> Vec<Vec<i8>> {
    let mut vec: Vec<Vec<i8>> = Vec::new();
    'x: for i in 0..2_usize.pow(10) {
        let v: Vec<i8> = format!("{:010b}", i)
                         .chars().map(|c| match c {'0' => -1, _ => 1}).collect();
        for (x,r1,r2,r3) in convex_rels5p(1,2,3,4,5) {
            let (x1,x2,x3) = (v[r1],v[r2],v[r3]);
            if x1 == x2 && x1 == x*x3 { continue 'x; }
        }
        vec.push(v);
    }
    return vec;
}

fn compatible(omega: & Vec<Vec<i8>>) -> CompatibleRels {
    let mut x = BTreeMap::new();
    for v in omega.iter() {
        let mut vec: Vec<Vec<i8>> = Vec::new();
        for u in omega.iter() {
            if (1..5).tuple_combinations::<(_,_,_)>().all(|(a,b,c)| v[cd(a+1,b+1,c+1)] == u[cd(a,b,c)]) {
                vec.push(u.to_vec());
            }
        }
        x.insert(v.to_vec(),vec);
    }
    return x;
}

fn check_concave(rels: & Vec<Vec<Crel>>, f: &mut Vec<i8>, cod: usize) -> bool {
    for (c,r1,r2,r3) in rels[cod].iter() {
        let (j1,j2,j3) = (f[*r1],f[*r2],f[*r3]);
        if j1 != 0 && j2 != 0 && j3 != 0 && (j1 == j2 && j2 == c*j3) {
            return false;
        }
    }
    return true
}

fn first_undef(n: usize, f: &mut Vec<i8>) -> Option<usize> {
    let mut j: usize = 1;
    while j+5-1 <= n {
        for (a,b,c) in (1..5+1).tuple_combinations::<(_,_,_)>() {
            if f[cd(a+j-1,b+j-1,c+j-1)] == 0 {
                return Some(j);
            }
        }
        j = j+1;
    }
    return None;
}

fn add(rels: &Vec<Vec<Crel>>, f: &mut Vec<i8>, hs: &mut Vec<usize>, added: usize) -> bool {
    if ! check_concave(rels, f, added) { return false; }
    let mut toadd: Vec<(usize,i8)> = Vec::new();
    for (c,r1,r2,r3) in rels[added].iter() {
        let (x1,x2,x3) = (f[*r1],f[*r2],f[*r3]);
        if x1 == 0 && x2 != 0 && x3 != 0 && x2 == c*x3 { toadd.push((*r1,-x2)); }
        if x1 != 0 && x2 == 0 && x3 != 0 && x1 == c*x3 { toadd.push((*r2,-x1)); }
        if x1 != 0 && x2 != 0 && x3 == 0 && x1 == x2   { toadd.push((*r3,-c*x1)); }
    }
    for (x,v) in toadd.iter() {
        f[*x] = *v;
        hs.push(*x);
        let b = add(rels, f, hs, *x);
        if ! b { return false; }
    }
    return true;
}

fn set_vec5(rels: &Vec<Vec<Crel>>, f: &mut Vec<i8>, hs: &mut Vec<usize>, j: usize, v: &Vec<i8>) -> bool {
    for (a,b,c) in (1..5+1).tuple_combinations::<(_,_,_)>() {
        let cod = cd(a+j-1,b+j-1,c+j-1);
        if f[cod] == 0 {
            f[cod] = v[cd(a,b,c)];
            hs.push(cod);
            let b = add(rels,f,hs,cod);
            if ! b {
                return false;
            }
        } else if f[cod] != v[cd(a,b,c)] {
            return false;
        }
    }
    return true;
}

fn get_vec5(f: &mut Vec<i8>, j: usize) -> Vec<i8> {
    let mut v = vec![0;10];
    for (a,b,c) in (1..5+1).tuple_combinations::<(_,_,_)>() {
        v[cd(a,b,c)] = f[cd(a+j-1,b+j-1,c+j-1)];
    }
    return v;
}

fn search(n: usize, rels: &Vec<Vec<Crel>>, comp: & CompatibleRels, f: &mut Vec<i8>, hs: &mut Vec<usize>, count: &mut i64) -> () {
    let oj = first_undef(n,f);
    if oj == None { *count = *count+1; }
    else {
        let j = oj.unwrap();
        let com = comp.get(&get_vec5(f,j-1)).unwrap();
        let u = get_vec5(f, j);
        for v in com.iter() {
            if (0..v.len()).all(|i| u[i] == 0 || u[i] == v[i]) {
                let k = hs.len();
                let b = set_vec5(rels, f, hs, j, v);
                if b { search(n, rels, comp, f, hs, count); }
                for i in k..hs.len() { f[hs[i]] = 0; }
                for _ in 0..hs.len() - k { hs.pop(); }
            }
        }
    }
}

pub fn main(){
    const N: usize = 9;
    let omega = concave5();
    let comp = compatible(& omega);
    let convexr = convex_rels5(N);
    let mut count: i64 = 0;
    for v in omega.iter() {
        if v[0] == 1 {
            let mut f: Vec<i8> = vec![0;N*(N-1)*(N-2)/6];
            let mut history: Vec<usize> = vec![];
            let b = set_vec5(&convexr, &mut f, &mut history, 1, v);
            if b { search(N, &convexr, &comp, &mut f, &mut history, &mut count); }
        }
    }
    println!("{}",count);
}
