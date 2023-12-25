use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_24_part1(INPUT);
    let elapsed_1 = now_1.elapsed();
    println!("part 1 - Result -> {}", part1);
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    // let now_2 = Instant::now();
    // let part2 = solve_day_24_part2(INPUT, (0, 1), (140, 139));
    // let elapsed_2 = now_2.elapsed();
    // println!("part2 - Result -> {}", part2);
    // println!("Part 2 - Elapsed: {:.2?}", elapsed_2);

    // // Benchmark
    // println!("\nBenchmark:");
    // let mut part1_v = Vec::<u128>::new();
    // let mut part2_v = Vec::<u128>::new();
    // for _ in 0..1000 {
    //     let now_1 = Instant::now();
    //     let part1 = solve_day_24_part1(INPUT);
    //     let elapsed_1 = now_1.elapsed();
    //     part1_v.push(elapsed_1.as_micros());
    //     let _ = part1;

    //     let now_2 = Instant::now();
    //     let part2 = solve_day_24_part2(INPUT);
    //     let elapsed_2 = now_2.elapsed();
    //     part2_v.push(elapsed_2.as_micros());
    //     let _ = part2;
    // }
    // println!(
    //     "Part 1 - Avarage: {:.2?}",
    //     part1_v.iter().sum::<u128>() / 1000 as u128
    // );
    // println!(
    //     "Part 2 - Avarage: {:.2?}",
    //     part2_v.iter().sum::<u128>() / 1000 as u128
    // );
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");
#[test]
fn example() {
    let part1 = solve_day_24_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 54);

    // let part2 = solve_day_24_part2(EXAMPLE_DATA_1);
    // assert_eq!(part2, 47);
}

fn solve_day_24_part1(input: &str) -> u64 {
    let edges: HashSet<(Box<str>, Box<str>)> = input
        .lines()
        .flat_map(|l| {
            let (a, bs) = l.trim().split_once(":").unwrap();
            bs.trim().split(" ").map(move |b| (a.into(), b.into()))
        })
        .collect();

    let mut edges_count: HashMap<(Box<str>, Box<str>), u32> = edges
        .iter()
        .map(|e| ((e.0.clone(), e.1.clone()), 0 as u32))
        .collect();

    let mut adj_grph = HashMap::<Box<str>, HashSet<Box<str>>>::new();

    edges.iter().for_each(|(a, b)| {
        if !adj_grph.contains_key(a) {
            adj_grph.insert(a.clone(), HashSet::<Box<str>>::new());
        }
        if !adj_grph.contains_key(b) {
            adj_grph.insert(b.clone(), HashSet::<Box<str>>::new());
        }
        adj_grph.entry(a.clone()).and_modify(|v| {
            v.insert(b.clone());
        });
        adj_grph.entry(b.clone()).and_modify(|v| {
            v.insert(a.clone());
        });
    });

    let nodes: Vec<Box<str>> = adj_grph.keys().cloned().collect();

    // for n in nodes.clone() {
    //     println!("node: {}", n.clone());
    // }

    println!("{} nodes", nodes.len());

    let mut count = 0;

    nodes.iter().for_each(|n| {
        most_frequent_in_shortest_paths(n.clone(), &adj_grph, &mut edges_count);
        count += 1;
        print!("..{}", count);
    });

    // print!("{:?}", edges_count);

    let mut edges_keys: Vec<_> = edges_count.keys().collect();

    // Sort the vector based on the values
    edges_keys.sort_by(|&a, &b| {
        edges_count
            .get(a)
            .unwrap()
            .cmp(edges_count.get(b).unwrap())
            .reverse()
    });

    // removed_edges.resize(3, &("ok".into(), "ok".into()));

    let mut uf = UnionFind::new(adj_grph.keys().cloned().collect());

    edges_keys.iter().skip(3).for_each(|(a, b)| {
        uf.union(a.clone(), b.clone());
    });

    // println!("Components: {}", uf.num_components());

    uf.sz
        .iter()
        .filter(|(_, &b)| b > 0)
        .fold(1, |a, (_, c)| a * c) as u64
}

fn most_frequent_in_shortest_paths(
    root: Box<str>,
    adj_grph: &HashMap<Box<str>, HashSet<Box<str>>>,
    edges_count: &mut HashMap<(Box<str>, Box<str>), u32>,
) {
    let mut reached_nodes = HashSet::<Box<str>>::new();
    reached_nodes.insert(root.clone());
    // filtro nodi raggiunti che stanno in reached_nodes
    // aggiungo a reached_nodes e incremento il count dei paths

    let mut paths = VecDeque::<Vec<Box<str>>>::new();
    paths.push_back(vec![root.clone()]);

    while let Some(path) = paths.pop_front() {
        let new_nodes: Vec<Box<str>> = adj_grph
            .get(path.last().unwrap())
            .unwrap()
            .iter()
            .filter(|&n| !reached_nodes.contains(n))
            .cloned()
            .collect();

        new_nodes.iter().for_each(|n| {
            reached_nodes.insert(n.clone());

            let new_path = [path.clone(), vec![n.clone()]].concat();

            new_path.windows(2).for_each(|a| {
                let mut k = (a.get(1).unwrap().clone(), a.get(0).unwrap().clone());
                if edges_count
                    .contains_key(&((a.get(0).unwrap().clone(), a.get(1).unwrap().clone())))
                {
                    k = (a.get(0).unwrap().clone(), a.get(1).unwrap().clone());
                }
                edges_count.insert(k.clone(), edges_count.get(&k).unwrap() + 1);
            });

            paths.push_back(new_path);
        })
    }
}

struct UnionFind {
    // size: usize,
    sz: HashMap<Box<str>, usize>,
    parents: HashMap<Box<str>, Box<str>>,
    num_components: usize,
}

impl UnionFind {
    fn new(nodes: Vec<Box<str>>) -> Self {
        Self {
            parents: nodes.iter().map(|n| (n.clone(), n.clone())).collect(),
            num_components: nodes.len(),
            sz: nodes.iter().map(|n| (n.clone(), 1)).collect(),
            // size: nodes.len(),
        }
    }

    fn find(&mut self, node: Box<str>) -> Box<str> {
        let mut root = node.clone();

        while root.clone() != self.parents.get(&root).unwrap().clone() {
            root = self.parents.get(&root).unwrap().clone();
        }

        let mut p = node.clone();
        while p != root.clone() {
            let next = self.parents.get(&p).unwrap().clone();
            self.parents.insert(p, root.clone());
            p = next.clone();
        }
        root
    }

    fn connected(&mut self, a: Box<str>, b: Box<str>) -> bool {
        self.find(a) == self.find(b)
    }

    // fn component_size(&self, node: &Box<str>) -> usize {
    //     self.sz.get(node).unwrap().clone()
    // }

    // fn size(&self) -> usize {
    //     self.size
    // }

    // fn num_components(&self) -> usize {
    //     self.num_components
    // }

    fn union(&mut self, p: Box<str>, q: Box<str>) {
        // These elements are already in the same group!
        if self.connected(p.clone(), q.clone()) {
            return;
        }

        let root1 = self.find(p);
        let root2 = self.find(q);

        // Merge smaller component/set into the larger one.
        let new_size = self.sz.get(&root2).unwrap() + self.sz.get(&root1).unwrap();
        if self.sz.get(&root1).unwrap() < self.sz.get(&root2).unwrap() {
            self.sz.insert(root2.clone(), new_size);
            self.parents.insert(root1.clone(), root2.clone());
            self.sz.insert(root1.clone(), 0);
        } else {
            self.sz.insert(root1.clone(), new_size);
            self.parents.insert(root2.clone(), root1.clone());
            self.sz.insert(root2.clone(), 0);
        }

        // Since the roots found are different we know that the
        // number of components/sets has decreased by one
        self.num_components -= 1;
    }
}

// fn solve_day_24_part2(input: &str) -> u64 {

//     0
// }
