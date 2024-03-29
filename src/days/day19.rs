use regex::Regex;
use shash::SHashSet;

use crate::aoc_base::Day;

pub struct Day19;

type Rot = [(usize, i8); 3];
const ROTS: [Rot; 24] = [[(0, 1), (1, 1), (2, 1)], [(0, 1), (1, -1), (2, -1)], [(0, -1), (1, 1), (2, -1)], [(0, -1), (1, -1), (2, 1)], [(0, 1), (2, 1), (1, -1)], [(0, 1), (2, -1), (1, 1)], [(0, -1), (2, 1), (1, 1)], [(0, -1), (2, -1), (1, -1)], [(1, 1), (2, 1), (0, 1)], [(1, 1), (2, -1), (0, -1)], [(1, -1), (2, 1), (0, -1)], [(1, -1), (2, -1), (0, 1)], [(1, 1), (0, 1), (2, -1)], [(1, 1), (0, -1), (2, 1)], [(1, -1), (0, 1), (2, 1)], [(1, -1), (0, -1), (2, -1)], [(2, 1), (0, 1), (1, 1)], [(2, 1), (0, -1), (1, -1)], [(2, -1), (0, 1), (1, -1)], [(2, -1), (0, -1), (1, 1)], [(2, 1), (1, 1), (0, -1)], [(2, 1), (1, -1), (0, 1)], [(2, -1), (1, 1), (0, 1)], [(2, -1), (1, -1), (0, -1)]];
fn _rots_gen() {
    let mut q = vec![];
    for i in 0..3 {
        for (d, v) in [(1, 1), (2, -1)] {
            let j = (i+d)%3;
            for vi in [1, -1] { for vj in [1, -1] {
                let k = 3 - i - j;
                let vk = v * vi * vj;
                q.push([(i, vi), (j, vj), (k, vk)]);
            }}
        }
    }

    println!("{:?}", &q);
}

type Pt = crate::util::pt::Pt<3>;
impl Pt {
    fn rotate(self, rot: Rot) -> Pt {
        let mut ans = [0; 3];
        for i in 0..3 {
            ans[rot[i].0] = self[i] * rot[i].1 as i64;
        }
        ans.into()
    }
    fn transform(self, tr: Transform) -> Pt {
        self.rotate(tr.0) + tr.1
    }
}

type Transform = (Rot, Pt);
fn compose(f: Transform, g: Transform) -> Transform {
    let mut rot = Rot::default();
    for i in 0..3 {
        let a = f.0[i];
        let b = g.0[a.0];
        rot[i] = (b.0, a.1 * b.1)
    }
    (rot, f.1.transform(g))
}

fn rotate_points(pts: &[Pt], rot: Rot) -> Vec<Pt> {
    pts.iter().map(|x| x.rotate(rot)).collect()
}

fn find_transform(a: &[Pt], b: &[Pt]) -> Option<Transform> {
    let s = b.iter().copied().collect::<SHashSet<_>>();
    
    for rot in ROTS {
        let a = rotate_points(a, rot);
        for &pa in &a { for &pb in b {
            let shift = pb - pa;
            if a.iter().map(|&x| x + shift).filter(|&x| s.contains(&x)).nth(11).is_some() {
                return Some((rot, shift));
            }
        }}
    }

    None
}

fn find_scanners(data: &Data) -> Vec<Transform> {
    let n = data.len();

    let mut ans = vec![(ROTS[0], Pt::default()); n];
    let mut vis = vec![false; n];
    let mut stk = vec![0];
    vis[0] = true;

    while let Some(u) = stk.pop() {
        for v in 0..n { if !vis[v] {
            if let Some(t) = find_transform(&data[v], &data[u]) {
                vis[v] = true;
                ans[v] = compose(t, ans[u]);
                stk.push(v);
            }
        }}
    }

    assert!(vis.iter().all(|&x| x));
    ans
}

type Data = Vec<Vec<Pt>>;

impl Day for Day19 {
    type Parsed = (Data, Vec<Transform>);

    type Output1 = usize;

    type Output2 = i64;

    fn num() -> usize {
        19
    }

    fn title() -> &'static str {
        "Beacon Scanner"
    }

    fn parse(input: &str) -> Self::Parsed {
        let re = Regex::new(r"\s*--- scanner \d+ ---\s*").unwrap();
        let data = re.split(input).skip(1).map(|section|
            section.lines().map(|ln| {
                let mut it = ln.split(',').map(|v| v.parse().unwrap());
                [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()].into()
            }).collect()
        ).collect();
        let transforms = find_scanners(&data);
        (data, transforms)
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let (data, transforms) = data;
        
        let mut beacons = SHashSet::default();
        for i in 0..data.len() {
            beacons.extend(data[i].iter().map(|&x| x.transform(transforms[i])));
        }

        beacons.len()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let (data, transforms) = data;

        let mut ans = 0i64;
        for i in 0..data.len() {
            for j in 0..i {
                ans = ans.max((transforms[i].1 - transforms[j].1).manhattan_distance())
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn test_input() {
        Day19::test(InputSource::Literal(TEST_INPUT), Some(79), Some(3621));
    }

    #[test]
    fn gmail() {
        Day19::test(InputSource::gmail(19), Some(428), Some(12140))
    }

    const TEST_INPUT: &str = indoc!{"
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401
        
        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390
        
        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562
        
        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596
        
        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
    "};
}