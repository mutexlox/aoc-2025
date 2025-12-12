use aoc_2025::util;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node(i64, i64, i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Edge {
    a: Node,
    b: Node,
}

impl Edge {
    fn dist(&self) -> f64 {
        (((self.a.0 - self.b.0).pow(2)
            + (self.a.1 - self.b.1).pow(2)
            + (self.a.2 - self.b.2).pow(2)) as f64)
            .sqrt()
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse to make a min-heap
        other.dist().partial_cmp(&self.dist()).unwrap()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct UnionFindNode {
    parent: Node,
    this: Node,
    size: usize,
}

impl UnionFindNode {
    fn new(node: &Node) -> UnionFindNode {
        Self {
            parent: *node,
            this: *node,
            size: 1,
        }
    }
}

struct UnionFind {
    nodes: HashMap<Node, UnionFindNode>,
}

impl UnionFind {
    fn new(nodes: &[Node]) -> UnionFind {
        let mut union_find = HashMap::new();
        for n in nodes {
            union_find.insert(*n, UnionFindNode::new(n));
        }
        Self { nodes: union_find }
    }

    fn find(&mut self, node: &Node) -> Option<Node> {
        let uf_node = *self.nodes.get(node)?;
        if uf_node.parent != *node {
            let parent = self.find(&uf_node.parent)?;
            self.nodes.get_mut(node)?.parent = parent;
            return Some(parent);
        }
        Some(uf_node.this)
    }

    // Returns Some(true) if we merged them; Some(false) if already the same set
    fn union(&mut self, a: &Node, b: &Node) -> Option<bool> {
        let mut a = self.find(a)?;
        let mut b = self.find(b)?;
        let mut uf_a = self.nodes[&a];
        let mut uf_b = self.nodes[&b];

        if uf_a == uf_b {
            return Some(false);
        }

        if uf_a.size < uf_b.size {
            (a, b) = (b, a);
            (uf_a, uf_b) = (uf_b, uf_a);
        }

        self.nodes.get_mut(&b)?.parent = uf_a.this;
        self.nodes.get_mut(&a)?.size += uf_b.size;
        Some(true)
    }

    fn product_of_largest_n(&mut self) -> usize {
        let nodes = self.nodes.keys().cloned().collect::<Vec<_>>();
        nodes
            .iter()
            .map(|n| {
                let r = self.find(n).unwrap();
                self.nodes[&r]
            })
            .unique()
            .map(|n| n.size)
            .sorted()
            .rev()
            .take(3)
            .product()
    }
}

// returns product of size of three largest circuits after |steps|
// and product of x-coordinates of last edge added
fn run_steps_of_kruskal(
    verts: &[Node],
    edges: &mut BinaryHeap<Edge>,
    steps: usize,
) -> (usize, i64) {
    let mut uf = UnionFind::new(verts);
    let mut i = 0;
    let mut forest = HashSet::new();
    let mut size = 0;
    let mut last_added = 0;
    while let Some(e) = edges.pop() {
        if i == steps {
            size = uf.product_of_largest_n();
        }
        if uf.find(&e.a).unwrap() != uf.find(&e.b).unwrap() {
            forest.insert(e);
            last_added = e.a.0 * e.b.0;
            uf.union(&e.a, &e.b).unwrap();
        }
        i += 1;
    }
    (size, last_added)
}

fn main() {
    let mut verts = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let coords = line
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(coords.len(), 3);
        verts.push(Node(coords[0], coords[1], coords[2]));
    }
    let mut edges = BinaryHeap::new();
    for (i, &v) in verts.iter().enumerate() {
        for &b in verts.iter().skip(i + 1) {
            edges.push(Edge { a: v, b, });
        }
    }
    let (p, last_added) = run_steps_of_kruskal(&verts, &mut edges, 1000);
    println!("{}", p);
    println!("{}", last_added);
}
