//!  Example usage:
//! 
//! ```
//! use hopcroft_karp::matching;
//! 
//! fn main() {
//!     let edges = vec![(0,10), (0,11), (0,12), (1,11), (2,12)];
//!     let res = matching(&edges);
//!     assert_eq!(res.len(), 3);
//! }
//! ```
//! 
//! ```        
//! use hopcroft_karp::{matching, matching_mapped};
//! 
//! fn main() {
//!     let edges = vec![("spiderman", "doc octopus"), ("spiderman", "sandman"), ("spiderman", "green goblin"),
//!                      ("silk", "doc octopus"), ("silk", "green goblin"),  ("daredevil", "sandman")];
//!     let res = matching(&edges);
//!     assert_eq!(res.len(), 3);
//! 
//!     let res = matching_mapped(&edges);
//!     assert_eq!(res.len(), 3);
//! }
//! ```
//! 

use std::{collections::VecDeque, hash::Hash};

use fxhash::{FxHashMap, FxHashSet};

type VertexSet<V> = FxHashSet<V>;
type Edge<V> = (V, V);

#[derive(PartialEq, Eq, Clone, Hash, Copy)]
enum Guarded<V> where V: Hash + Copy + Eq {
    GUARD,
    VALUE(V)
}

impl<V> Guarded<V> where V: Hash + Copy + Eq {
    fn vertex<'a>(&'a self) -> &'a V {
        match self {
            Guarded::GUARD => panic!(),
            Guarded::VALUE(u) => u,
        }
    }
}

const INFINITY:usize = usize::MAX;


struct HopcroftKarp<V> where V: Hash + Copy + Eq {
    pair_left: FxHashMap<V, Guarded<V>>,
    pair_right: FxHashMap<V, Guarded<V>>,
    distance:FxHashMap<Guarded<V>, usize>,
    size:usize,
    max_size:usize
}

impl<V> HopcroftKarp<V> where V: Hash + Copy + Eq {
    fn new(graph:&BGraph<V>) -> Self {
        let pair_left = graph.left.iter().map(|u| (*u, Guarded::GUARD)).collect();
        let pair_right = graph.right.iter().map(|v| (*v, Guarded::GUARD)).collect();
        let distance = FxHashMap::default();
        let max_size = std::cmp::min(graph.left.len(), graph.right.len());
        HopcroftKarp::<V>{ pair_left, pair_right, distance, size:0, max_size }
    }

    fn compute(mut self, graph:&BGraph<V>) -> Vec<Edge<V>> {
        while self.bfs(&graph) && self.size < self.max_size {
            for u in &graph.left {
                if self.pair_left[u] == Guarded::GUARD {
                    if self.dfs(&Guarded::VALUE(*u), &graph) {
                        self.size += 1;
                    }
                }
            }
        }
        self.pair_left.into_iter().filter(|(_,v)| v != &Guarded::GUARD ).map(|(u,v)| (u, *v.vertex())).collect()
    }

    fn compute_size(mut self, graph:&BGraph<V>) -> usize {
        while self.bfs(&graph) && self.size < self.max_size {
            for u in &graph.left {
                if self.pair_left[u] == Guarded::GUARD {
                    if self.dfs(&Guarded::VALUE(*u), &graph) {
                        self.size += 1;
                    }
                }
            }
        }
        self.size
    }

    fn bfs(&mut self, graph:&BGraph<V>) -> bool {
        let mut queue:VecDeque<Guarded<V>> = VecDeque::default();
        
        for u in &graph.left {
            let u_guarded = Guarded::VALUE(*u);
            if self.pair_left[u] == Guarded::GUARD {
                self.distance.insert(u_guarded, 0);
                queue.push_back(u_guarded);
            } else {
                self.distance.insert(u_guarded, INFINITY);
            }
        }

        // Imagine right_guard as a vertex (on the left) which is connected 
        // to all unmatched vertices on the right
        self.distance.insert(Guarded::GUARD, INFINITY);

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            debug_assert!(self.distance.contains_key(&u));

            if self.distance[&u] < self.distance[&Guarded::GUARD] {
                for v in graph.neighbours_guarded(&u) {
                    let v_pair = self.pair_right[&v];
                    if self.distance[&v_pair] == INFINITY {
                        self.distance.insert(v_pair, self.distance[&u] + 1);
                        queue.push_back(v_pair);
                    }
                }
            }
        }

        return self.distance[&Guarded::GUARD] != INFINITY;
    }   

    fn dfs(&mut self, u_guarded:&Guarded<V>, graph:&BGraph<V>) -> bool {
        if let Guarded::VALUE(u) = u_guarded  {
            let u_neighbours:Vec<_> = { 
                graph.neighbours(u).clone().collect()
            };
            for v in u_neighbours {
                let v_pair = self.pair_right[&v];
                if self.distance[&v_pair] == self.distance[u_guarded]+1 {
                    if self.dfs(&v_pair, graph) {
                        // Match up v and u
                        self.pair_right.insert(*v, *u_guarded);
                        self.pair_left.insert(*u ,Guarded::VALUE(*v));
                        return true;
                    }
                }
            }
            self.distance.insert(*u_guarded, INFINITY);
            false            
        } else {
            true
        }
    }       
}

struct BGraph<V> {
    left: VertexSet<V>,
    right: VertexSet<V>,
    adj:FxHashMap<V,VertexSet<V>>,
}

impl BGraph<usize> {
    fn new_mapped<V>(edges:&Vec<Edge<V>>) -> (BGraph<usize>, FxHashMap<usize, V>) where V: Hash + Copy + Eq {
        let mut orig_left:FxHashSet<V> = FxHashSet::default();
        let mut orig_right:FxHashSet<V> = FxHashSet::default();

        let mut left = FxHashSet::default();
        let mut right = FxHashSet::default();

        let mut adj:FxHashMap<usize,VertexSet<usize>> = FxHashMap::default();

        // Collect vertices
        for (u,v) in edges {
            orig_left.insert(*u);
            orig_right.insert(*v);
        }

        // Create mapping
        let mut mapping:FxHashMap<V, usize> = FxHashMap::default();
        let mut back_mapping:FxHashMap<usize, V> = FxHashMap::default();
        let mut id = 0;
        for u in orig_left {
            mapping.insert(u, id);
            back_mapping.insert(id, u);
            left.insert(id);
            id += 1;
        }
        for u in orig_right {
            mapping.insert(u, id);
            back_mapping.insert(id, u);
            right.insert(id);
            id += 1;
        }        

        // Construct adjacency lists
        for (u,v) in edges {
            let u_map = *mapping.get(u).unwrap();
            let v_map = *mapping.get(v).unwrap();

            adj.entry(u_map).or_default().insert(v_map);
            adj.entry(v_map).or_default().insert(u_map);
            left.insert(u_map);
            right.insert(v_map);
        }

        (BGraph { left, right, adj}, back_mapping)
    }
}

impl<V> BGraph<V> where V: Hash + Copy + Eq {
    fn new(edges:&Vec<Edge<V>>) -> BGraph<V> {
        let mut left = FxHashSet::default();
        let mut right = FxHashSet::default();
        let mut adj:FxHashMap<V,VertexSet<V>> = FxHashMap::default();
        for &(u,v) in edges {
            adj.entry(u).or_default().insert(v);
            adj.entry(v).or_default().insert(u);
            left.insert(u);
            right.insert(v);
        }

        if left.intersection(&right).count() > 0 {
            panic!("Provided graph is not bipartite!");
        }

        BGraph { left, right, adj}
    }

    fn compute(self) -> Vec<Edge<V>> {
        let hk = HopcroftKarp::new(&self);
        hk.compute(&self)
    }

    fn compute_size(self) -> usize {
        let hk = HopcroftKarp::new(&self);
        hk.compute_size(&self)
    }    

    fn compute_bounded(self, bound: usize) -> Vec<Edge<V>> {
        let mut hk = HopcroftKarp::new(&self);
        hk.max_size = std::cmp::min(bound, hk.max_size);
        hk.compute(&self)
    }

    fn compute_bounded_size(self, bound: usize) -> usize {
        let mut hk = HopcroftKarp::new(&self);
        hk.max_size = std::cmp::min(bound, hk.max_size);
        hk.compute_size(&self)
    }    

    fn neighbours<'a>(&'a self, u:&V) -> std::collections::hash_set::Iter<V>  {
        self.adj[u].iter()
    }

    fn neighbours_guarded<'a>(&'a self, guarded:&Guarded<V>) -> std::collections::hash_set::Iter<V>  {
        match guarded {
            Guarded::GUARD => panic!(),
            Guarded::VALUE(u) => self.adj[u].iter(),
        }
    }    
}

pub fn matching<V>(edges:&Vec<Edge<V>>) -> Vec<Edge<V>> where V: Hash + Copy + Eq {
    BGraph::new(edges).compute()
}

pub fn matching_size<V>(edges:&Vec<Edge<V>>) -> usize where V: Hash + Copy + Eq {
    BGraph::new(edges).compute_size()
}

pub fn matching_mapped<V>(edges:&Vec<Edge<V>>) -> Vec<Edge<V>> where V: Hash + Copy + Eq {
    let (graph, mapping) = BGraph::new_mapped(edges);
    let res = graph.compute();

    res.iter().map(|(u,v)| (mapping[u], mapping[v])).collect()
}

pub fn matching_mapped_size<V>(edges:&Vec<Edge<V>>) -> usize where V: Hash + Copy + Eq {
    let (graph, _) = BGraph::new_mapped(edges);
    graph.compute_size()
}

pub fn bounded_matching<V>(edges:&Vec<Edge<V>>, bound:usize) -> Vec<Edge<V>> where V: Hash + Copy + Eq {
    BGraph::new(edges).compute_bounded(bound)
}

pub fn bounded_matching_mapped<V>(edges:&Vec<Edge<V>>, bound:usize) -> Vec<Edge<V>> where V: Hash + Copy + Eq {
    let (graph, mapping) = BGraph::new_mapped(edges);
    let res = graph.compute_bounded(bound);

    res.iter().map(|(u,v)| (mapping[u], mapping[v])).collect()
}

pub fn bounded_matching_mapped_size<V>(edges:&Vec<Edge<V>>, bound:usize) -> usize where V: Hash + Copy + Eq {
    let (graph, _) = BGraph::new_mapped(edges);
    graph.compute_bounded_size(bound)
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::{SeedableRng, RngCore}; 
    use rand_chacha::ChaChaRng; 

    #[test]
    fn test_basic() {        
        let edges = vec![(0,10), (0,11), (0,12), (1,11), (2,12)];
        let res = matching(&edges);
        assert_eq!(res.len(), 3);
        let expected = vec![(0,10), (1,11), (2,12)];
        assert_eq!(res.iter().copied().collect::<FxHashSet<(i32,i32)>>(),
                   expected.iter().copied().collect::<FxHashSet<(i32,i32)>>() );

        let edges = vec![(0,10), (0,11), (0,12), (0,13)];
        let res = matching(&edges);
        assert_eq!(res.len(), 1);
        assert!(edges.contains(&res[0]));
    }

    #[test]
    fn test_random_perfect() {
        // Pseudo-random test: perfect matching exists
        let n:usize = 100;
        let mut rng = ChaChaRng::from_entropy();

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

        let res = matching(&edges);
        assert_eq!(res.len(), n);
        assert_eq!(res.len(), matching_size(&edges));
    }

    #[test]
    fn test_random_lopsided() {
        let n:usize = 100;
        let mut rng = ChaChaRng::from_entropy();

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

        let res = matching(&edges);
        assert_eq!(res.len(), n);
        assert_eq!(res.len(), matching_size(&edges));        
    }

    #[test]
    fn test_edge_cases() {
        let edges:Vec<(u8,u8)> = vec![];
        let res = matching(&edges);
        assert_eq!(res.len(), 0);

        let edges = vec![(0,1)];
        let res = matching(&edges);
        assert_eq!(res.len(), 1);

        let edges = vec![(0,1), (0,1)];
        let res = matching(&edges);
        assert_eq!(res.len(), 1);   
    }

    #[test]
    #[should_panic]
    fn test_non_bipartite() {
        matching(&vec![(0,1),(0,2),(1,2)]);
    }

    #[test]
    fn test_spiderman() {
        let edges = vec![("spiderman", "doc octopus"), ("spiderman", "sandman"), ("spiderman", "green goblin"),
                         ("silk", "doc octopus"), ("silk", "green goblin"),  ("daredevil", "sandman")];
        let res = matching(&edges);
        assert_eq!(res.len(), 3);
    }    
}
