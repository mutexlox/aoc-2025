use aoc_2025::util;
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
    parent: Option<Rc<RefCell<Node>>>,
    size: usize,
}

impl Node {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
            parent: None,
            size: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    a: Rc<RefCell<Node>>,
    b: Rc<RefCell<Node>>,
}

impl Edge {
    fn dist(&self) -> f64 {
        let a = self.a.borrow();
        let b = self.b.borrow();
        (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt()
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

fn find(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let parent = { node.borrow().parent.clone() };
    if let Some(parent) = parent {
        let parent = find(parent.clone());
        node.borrow_mut().parent = Some(parent.clone());
        parent
    } else {
        node
    }
}

// Returns true if we merged them; false if already the same set
fn union(a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) -> bool {
    let mut a = find(a);
    let mut b = find(b);

    if a == b {
        return false;
    }

    if a.borrow().size < b.borrow().size {
        (a, b) = (b, a);
    }

    b.borrow_mut().parent = Some(a.clone());
    a.borrow_mut().size += b.borrow().size;
    true
}

fn product_of_largest_n(nodes: &[Rc<RefCell<Node>>], n: usize) -> usize {
    nodes
        .iter()
        .map(|n| find(n.clone()))
        .unique_by(|n| {
            let n = n.borrow();
            (n.x, n.y, n.z)
        })
        .map(|n| n.borrow().size)
        .sorted()
        .rev()
        .take(n)
        .product()
}

// returns product of size of three largest circuits after |steps|
// and product of x-coordinates of last edge added
fn run_steps_of_kruskal(
    verts: &[Rc<RefCell<Node>>],
    edges: &mut BinaryHeap<Edge>,
    steps: usize,
) -> (usize, i64) {
    let mut i = 0;
    let mut size = 0;
    let mut last_added = 0;
    while let Some(e) = edges.pop() {
        if i == steps {
            size = product_of_largest_n(verts, 3);
        }
        if find(e.a.clone()) != find(e.b.clone()) {
            last_added = e.a.borrow().x * e.b.borrow().x;
            union(e.a, e.b);
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
        verts.push(Rc::new(RefCell::new(Node::new(
            coords[0], coords[1], coords[2],
        ))));
    }
    let mut edges = BinaryHeap::new();
    for (i, v) in verts.iter().enumerate() {
        for b in verts.iter().skip(i + 1) {
            edges.push(Edge {
                a: v.clone(),
                b: b.clone(),
            });
        }
    }
    let (p, last_added) = run_steps_of_kruskal(&verts, &mut edges, 1000);
    println!("{}", p);
    println!("{}", last_added);
}
