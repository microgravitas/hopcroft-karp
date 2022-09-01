[<img alt="crates.io" src="https://img.shields.io/crates/v/hopcroft-karp?style=flat-square"/>](https://crates.io/crates/hopcroft-karp)
[<img alt="github" src="https://img.shields.io/badge/github-hopcroft--karp-ffdd55?style=flat-square&logo=github"/>](https://github.com/microgravitas/hopcroft-karp)

This crate implements the <a href="https://en.wikipedia.org/wiki/Hopcroft%E2%80%93Karp_algorithm">Hopcroft-Karp algorithm</a> to find maximum unweighted matchings in bipartite graph. The crate provides the function `hopcroft_karp::matching` which
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

`matching` is generic over the vertex type, the trait bounds are `Hash + Copy + Eq`. For types where the copy operation
is potentially expensive (e.g. strings) the crate provides the function `hopcrof_karp::matching_mapped` which internally
maps the vertices onto integers and mostly avoids copying the type. 

```rs
use hopcroft_karp::{matching, matching_mapped};

fn main() {
    let edges = vec![("spiderman", "doc octopus"), ("spiderman", "sandman"), ("spiderman", "green goblin"),
                     ("silk", "doc octopus"), ("silk", "green goblin"),  ("daredevil", "sandman")];
    let res = matching(&edges);
    assert_eq!(res.len(), 3);

    // For types where copying is expensive, use this instead
    let res = matching_mapped(&edges);
    assert_eq!(res.len(), 3);
}
```