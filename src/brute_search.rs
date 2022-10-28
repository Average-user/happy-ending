// Brute forces a search.

type Crel = (i8,usize,usize,usize);
type Triangle = (usize,usize,usize);

fn cd(a: usize, b: usize, c: usize) -> usize {
    return ((c-1)*(c-2)*(c-3))/6 + ((b-1)*(b-2))/2 + a -1;
}

fn cdt(t: Triangle) -> usize {
    let (a,b,c) = t;
    return cd(a,b,c);
}

fn convex_rels5(n: usize) -> Vec<Vec<Crel>> {
    let mut convexr: Vec<Vec<Crel>> = Vec::new();
    for _ in 0..n*(n-1)*(n-2)/6 {
        convexr.push(vec![]);
    }
    for a in 1..n+1 {
        for b in a+1..n+1 {
            for c in b+1..n+1 {
                for d in c+1..n+1 {
                    for e in d+1..n+1 {
                        convexr[cd(a,b,c)].push((1, cd(a,b,c),cd(b,c,d),cd(c,d,e)));
                        convexr[cd(b,c,d)].push((1, cd(a,b,c),cd(b,c,d),cd(c,d,e)));
                        convexr[cd(c,d,e)].push((1, cd(a,b,c),cd(b,c,d),cd(c,d,e)));

                        convexr[cd(a,b,c)].push((-1,cd(a,b,c),cd(b,c,e),cd(a,d,e)));
                        convexr[cd(b,c,e)].push((-1,cd(a,b,c),cd(b,c,e),cd(a,d,e)));
                        convexr[cd(a,d,e)].push((-1,cd(a,b,c),cd(b,c,e),cd(a,d,e)));

                        convexr[cd(a,b,d)].push((-1,cd(a,b,d),cd(b,d,e),cd(a,c,e)));
                        convexr[cd(b,d,e)].push((-1,cd(a,b,d),cd(b,d,e),cd(a,c,e)));
                        convexr[cd(a,c,e)].push((-1,cd(a,b,d),cd(b,d,e),cd(a,c,e)));

                        convexr[cd(a,c,d)].push((-1,cd(a,c,d),cd(c,d,e),cd(a,b,e)));
                        convexr[cd(c,d,e)].push((-1,cd(a,c,d),cd(c,d,e),cd(a,b,e)));
                        convexr[cd(a,b,e)].push((-1,cd(a,c,d),cd(c,d,e),cd(a,b,e)));
                    }
                }
            }
        }
    }
    return convexr;
}

fn check_concave(rels: & Vec<Vec<Crel>>, f: &mut Vec<i8>, t: Triangle) -> bool {
    for r in rels[cdt(t)].iter() {
        let (c,r1,r2,r3) = r;
        let (j1,j2,j3) = (f[*r1],f[*r2],c*f[*r3]);
        if j1 != 0 && j2 != 0 && j3 != 0 && (j1 == j2 && j2 == j3) {
            return false;
        }
    }
    return true
}

fn first_undef(n: usize, f: &mut Vec<i8>) -> Option<Triangle> {
    for x in 1..n+1 {
        for y in x+1..n+1 {
            for z in y+1..n+1 {
                if 0 == f[cd(x,y,z)] {
                    return Some((x,y,z));
                }
            }
        }
    }
    return None;
}

fn search(n: usize, rels: & Vec<Vec<Crel>>, f: &mut Vec<i8>, count: &mut i64) -> () {
    let t1 = first_undef(n,f);
    if t1 == None {
        *count = *count+1;
    } else {
        let t = t1.unwrap();
        f[cdt(t)] = 1;
        if check_concave(rels,f,t) {
            search(n,rels,f,count);
        }
        f[cdt(t)] = -1;
        if check_concave(rels,f,t) {
            search(n,rels,f,count);
        }
        f[cdt(t)] = 0;
    }
}

fn main(){
    const N: usize = 8;
    let convexr = convex_rels5(N);
    let mut count: i64 = 0;
    let mut f: Vec<i8> = vec![0;N*(N-1)*(N-2)/6];
    f[cd(1,2,3)] = 1;
    search(N, &convexr, &mut f, &mut count);
    println!("{}",count);
}
