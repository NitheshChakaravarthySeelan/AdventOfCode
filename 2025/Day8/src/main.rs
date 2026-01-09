use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Edge {
    dist_sq: i64,
    u: usize,
    v: usize,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        // Path compression
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            // Union by size
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}

fn main() {
    // 1. Read and parse input
    let file =
        File::open("/home/nithesh/WindowsDrive/Coding/Rust/AdventOfCode/2025/Day8/src/input.txt")
            .expect("Could not open input.txt");
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let coords: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        points.push(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let n = points.len();

    // 2. Generate all pairs and calculate squared distances
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in i + 1..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            edges.push(Edge {
                dist_sq,
                u: i,
                v: j,
            });
        }
    }

    // 3. Sort edges by distance (shortest first)
    edges.sort_by_key(|e| e.dist_sq);

    // 4. Initialize Union-Find and connect the first 1000 shortest pairs
    let mut dsu = DSU::new(n);
    let connections_to_make = 1000.min(edges.len());

    for i in 0..connections_to_make {
        dsu.union(edges[i].u, edges[i].v);
    }

    // 5. Collect sizes of all unique circuits
    let mut circuit_sizes = Vec::new();
    for i in 0..n {
        if dsu.parent[i] == i {
            circuit_sizes.push(dsu.size[i]);
        }
    }

    // 6. Sort sizes descending and multiply top 3
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    let mut result: u128 = 1; // Use u128 to prevent overflow on product
    let top_three = &circuit_sizes[0..3.min(circuit_sizes.len())];

    for &s in top_three {
        result *= s as u128;
    }

    println!("Top 3 circuit sizes: {:?}", top_three);
    println!("Final product: {}", result);
}
