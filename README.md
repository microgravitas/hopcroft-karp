[<img alt="crates.io" src="https://img.shields.io/crates/v/hopcroft-karp?style=flat-square"/>](https://crates.io/crates/hopcroft-karp)
[<img alt="github" src="https://img.shields.io/badge/github-hopcroft--karp-ffdd55?style=flat-square&logo=github"/>](https://github.com/microgravitas/hopcroft-karp)

This crate implements the <a href="https://en.wikipedia.org/wiki/Hopcroft%E2%80%93Karp_algorithm">Hopcroft-Karp algorithm</a> to find maximum unweighted matchings in bipartite graph. The crate exposes a single function `hopcroft_karp::matching` which
takes as input a vector of edges (encoding the bipartite graph) and returns a maximum matching as a vector of edges.

 Example usage:

```rs
    use hopcroft_karp::matching;

    fn main() {
        let edges = vec![(0,10), (0,11), (0,12), (1,11), (2,12)];
        let res = matching(edges);
        assert_eq!(res.len(), 3);
    }
```