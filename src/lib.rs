use std::{mem, time::Instant};

use igraph::{
    igraph_add_edge, igraph_add_edges, igraph_empty, igraph_t, igraph_vector_int_init,
    igraph_vector_int_init_int, igraph_vector_int_set, igraph_vector_int_t,
};
use webgraph::{
    prelude::BvGraph,
    traits::{RandomAccessGraph, RandomAccessLabeling, SequentialLabeling},
};

fn webgraph_to_igraph(graph_filename: &str, directed: bool) -> igraph_t {
    let graph = BvGraph::with_basename(graph_filename).load().unwrap();

    let instant = Instant::now();

    let mut igraph = igraph_t::new(graph.num_nodes(), directed);

    eprintln!(
        "Created empty igraph with {} nodes in {:?}",
        graph.num_nodes(),
        instant.elapsed()
    );

    let mut dimvector = igraph_vector_int_t::with_capacity(graph.num_arcs() as usize * 2);

    let mut i = 0;
    for u in 0..graph.num_nodes() {
        for v in graph.successors(u) {
            // igraph.add_edge(u as i64, v as i64);
            dimvector.set(i, u as i64);
            i += 1;
            dimvector.set(i, v as i64);
            i += 1;
        }
    }

    eprintln!(
        "Prepared edge list with {} edges in {:?}",
        graph.num_arcs(),
        instant.elapsed()
    );

    igraph.add_edges_from_vector(&dimvector);
    eprintln!("Converted webgraph to igraph in {:?}", instant.elapsed());

    igraph
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_webgraph_to_igraph() {
        let igraph = webgraph_to_igraph("/Users/mn/Data/bitcoin/pg/pg", true);
        eprintln!("Mean degree {}", igraph.mean_degree(true));
        igraph.write_graph_graphml("pg.graphml");
    }
}
