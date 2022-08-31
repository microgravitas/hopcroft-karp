use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};

type Vertex = usize;
type VertexSet = FxHashSet<Vertex>;
type Edge = (Vertex, Vertex);

// RIGHT_GUARD lies on the left and is connected to 
// all unmatched vertices on the right
const RIGHT_GUARD:usize = usize::MAX;

// LEFT_GUARD lies on the right and is connected to 
// all unmatched vertices on the left
const LEFT_GUARD:usize = usize::MAX;

const INFINITY:usize = usize::MAX;


struct HopcroftKarp {
    pair_left: FxHashMap<Vertex, Vertex>,
    pair_right: FxHashMap<Vertex, Vertex>,
    distance:FxHashMap<Vertex, usize>
}

impl HopcroftKarp {
    fn compute(mut self, graph:&BGraph) -> Vec<Edge> {
        while self.bfs(&graph) {
            for u in &graph.left {
                if !self.pair_left[u] != LEFT_GUARD {
                    self.dfs(u, &graph);
                }
            }
        }
        
        self.pair_left.into_iter().collect()
    }

    fn bfs(&mut self, graph:&BGraph) -> bool {
        let mut queue:VecDeque<Vertex> = VecDeque::default();
        
        for u in &graph.left {
            if !self.pair_left.contains_key(u) {
                self.distance.insert(*u, 0);
                queue.push_back(*u);
            } else {
                self.distance.insert(*u, INFINITY);
            }
        }

        // Imagine right_guard as a vertex (on the left) which is connected 
        // to all unmatched vertices on the right
        self.distance.insert(RIGHT_GUARD, INFINITY);

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            debug_assert!(self.distance.contains_key(&u));
            if self.distance[&u] < self.distance[&RIGHT_GUARD] {
                for v in graph.neighbours(&u) {
                    let v_pair = self.pair_right[&v];
                    if self.distance[&v_pair] == INFINITY {
                        self.distance.insert(v_pair, self.distance[&u] + 1);
                        queue.push_back(v_pair);
                    }
                }
            }
        }

        return self.distance[&RIGHT_GUARD] != INFINITY;
    }   

    fn dfs(&mut self, u:&Vertex, graph:&BGraph) -> bool {
        debug_assert!(graph.left.contains(u));
        if *u != RIGHT_GUARD  {
            let u_neighbours:Vec<_> = { 
                graph.neighbours(u).clone().collect()
            };
            for v in u_neighbours {
                let v_pair = self.pair_right[&v];
                if self.distance[&v_pair] == self.distance[u]+1 {
                    if self.dfs(&v_pair, graph) {
                        // Match up v and u
                        self.pair_right.insert(*v, *u);
                        self.pair_left.insert(*u ,*v);
                        return true;
                    }
                }
            }
            self.distance.insert(*u, INFINITY);
            false            
        } else {
            true
        }
    }       
}

struct BGraph {
    left: VertexSet,
    right: VertexSet,
    adj:FxHashMap<Vertex,VertexSet>,
}

impl BGraph {
    fn new(edges:Vec<Edge>) -> BGraph {
        let mut left = FxHashSet::default();
        let mut right = FxHashSet::default();
        let mut adj:FxHashMap<Vertex,VertexSet> = FxHashMap::default();
        for &(u,v) in &edges {
            adj.entry(u).or_default().insert(v);
            adj.entry(v).or_default().insert(u);
            left.insert(u);
            right.insert(v);
        }

        BGraph { left, right, adj}
    }

    fn compute(self) -> Vec<Edge> {
        let pair_left = self.left.iter().map(|u| (*u, LEFT_GUARD)).collect();
        let pair_right = self.right.iter().map(|v| (*v, RIGHT_GUARD)).collect();
        let distance = FxHashMap::default();
        let hk = HopcroftKarp{ pair_left, pair_right, distance };

        hk.compute(&self)
    }

    fn neighbours<'a>(&'a self, u:&Vertex) -> std::collections::hash_set::Iter<usize>  {
        self.adj[u].iter()
    }
}


pub fn matching(edges:Vec<Edge>) -> Vec<Edge> {
    BGraph::new(edges).compute()
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::{seq::SliceRandom, SeedableRng, RngCore}; // 0.6.5
    use rand_chacha::ChaChaRng; // 0.1.1

    #[test]
    fn test_basic() {        
        let edges = vec![(0,10), (0,11), (0,12), (1,11), (2,12)];
        let res = matching(edges);
        assert_eq!(res.len(), 3);

        let edges = vec![(0,10), (0,11), (0,12), (0,13)];
        let res = matching(edges);
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_random_perfect() {
        // Pseudo-random test: perfect matching exists
        let n:usize = 100;
        let seed = [4; 32];
        let mut rng = ChaChaRng::from_seed(seed);

        let mut edges = Vec::default();
        let mut edges_set = FxHashSet::default();
        for u in 0..n {
            edges.push((u, n+u));
            edges_set.insert((n , n+u));
        }

        for _ in 0..2*n {
            let u = rng.next_u64() as usize % n;
            let v = n + rng.next_u64() as usize % n;
            if !edges_set.contains(&(u,v)) {
                edges.push((u,v));
                edges_set.insert((u,v));
            }
        }

        let res = matching(edges);
        assert_eq!(res.len(), n);
    }

    #[test]
    fn test_random_lopsided() {
        let n:usize = 100;
        let seed = [4; 32];
        let mut rng = ChaChaRng::from_seed(seed);

        let mut edges = Vec::default();
        let mut edges_set = FxHashSet::default();
        for u in 0..n {
            edges.push((u, n+u));
            edges_set.insert((n , n+u));
        }

        for _ in 0..2*n {
            let u = rng.next_u64() as usize % n;
            let v = n + rng.next_u64() as usize % (2*n);
            if !edges_set.contains(&(u,v)) {
                edges.push((u,v));
                edges_set.insert((u,v));
            }
        }

        let res = matching(edges);
        assert_eq!(res.len(), n);
    }

    #[test]
    fn test_edge_cases() {
        let edges = vec![];
        let res = matching(edges);
        assert_eq!(res.len(), 0);

        let edges = vec![(0,1)];
        let res = matching(edges);
        assert_eq!(res.len(), 1);        
    }
}
