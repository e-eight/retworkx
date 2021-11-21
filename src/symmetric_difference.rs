// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

use crate::{digraph, graph, NodeIndex, StablePyGraph};

use hashbrown::{HashMap, HashSet};

use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use petgraph::{algo, EdgeType};

use pyo3::prelude::*;
use pyo3::Python;

fn symmetric_difference<Ty: EdgeType>(
    py: Python,
    first: &StablePyGraph<Ty>,
    second: &StablePyGraph<Ty>,
) -> PyResult<StablePyGraph<Ty>> {
    let mut first_nodes_set = first.node_references().cloned();
    let mut second_nodes_set: HashSet<_> = second.node_references().cloned();
    let nodes_symm_diff: HashSet<_> = first_nodes_set
        .symmetric_difference(&second_nodes_set)
        .collect();
    if !nodes_symm_diff.is_empty() {
        // return Err(PyIndexError::new_err(
        //     "The two graphs do not have the same nodes.",
        // ));
    }

    let mut final_graph = StablePyGraph::<Ty>::with_capacity(
        first.node_count(),
        first.edge_count() + second.edge_count(),
    );
    let mut node_map: HashMap<NodeIndex, NodeIndex> =
        HashMap::with_capacity(first.node_count());
    for node_index in first.node_indices() {
        let node = first[node_index].clone_ref(py);
        let new_index = final_graph.add_node(node);
        node_map.insert(node_index, new_index);
    }

    let mut first_edges_set: HashSet<_> =
        first.edge_references().cloned().collect();
    let mut second_edges_set: HashSet<_> =
        second.edge_references().cloned().collect();

    for edge in first_edges_set.symmetric_difference(&second_edges_set) {
        let &source = node_map.get(&edge.source()).unwrap();
        let &target = node_map.get(&edge.target()).unwrap();
        let weight = edge.weight();
        final_graph.add_edge(source, target, weight.clone_ref(py));
    }

    Ok(final_graph)
}

/// Return a new PyGraph by forming the symmetric difference from two input
/// PyGraph objects
///
/// :param PyGraph first: The first undirected graph object
/// :param PyGraph second: The second undirected graph object
///
/// :returns: A new PyGraph object that is the cartesian product of ``first``
///     and ``second``.
///     It's worth noting the weight/data payload objects are
///     passed by reference from ``first`` and ``second`` to this new object.
/// :rtype: PyGraph
///
/// .. jupyter-execute::
///
///   import retworkx.generators
///   from retworkx.visualization import mpl_draw
///
///   graph_1 = retworkx.generators.path_graph(3)
///   graph_2 = retworkx.generators.mesh_graph(3)
///   graph_sym_diff = retworkx.graph_symmetric_difference(graph_1, graph_2)
///   mpl_draw(graph_sym_diff)
#[pyfunction()]
#[pyo3(text_signature = "(first, second, /)")]
fn graph_symmetric_difference(
    py: Python,
    first: &graph::PyGraph,
    second: &graph::PyGraph,
) -> PyResult<graph::PyGraph> {
    let out_graph = symmetric_difference(py, &first.graph, &second.graph)?;

    Ok(graph::PyGraph {
        graph: out_graph,
        multigraph: true,
        node_removed: false,
    })
}

/// Return a new PyDiGraph by forming the cartesian product from two input
/// PyDiGraph objects
///
/// :param PyDiGraph first: The first undirected graph object
/// :param PyDiGraph second: The second undirected graph object
///
/// :returns: A new PyDiGraph object that is the cartesian product of ``first``
///     and ``second``.
///     It's worth noting the weight/data payload objects are
///     passed by reference from ``first`` and ``second`` to this new object.
/// :rtype: PyDiGraph
///
/// .. jupyter-execute::
///
///   import retworkx.generators
///   from retworkx.visualization import mpl_draw
///
///   graph_1 = retworkx.generators.directed_path_graph(2)
///   graph_2 = retworkx.generators.directed_path_graph(3)
///   graph_sym_diff = retworkx.digraph_symmetric_difference(graph_1, graph_2)
///   mpl_draw(graph_sym_diff)
#[pyfunction()]
#[pyo3(text_signature = "(first, second, /)")]
fn digraph_symmetric_difference(
    py: Python,
    first: &digraph::PyDiGraph,
    second: &digraph::PyDiGraph,
) -> PyResult<digraph::PyDiGraph> {
    let out_graph = symmetric_difference(py, &first.graph, &second.graph)?;

    Ok(digraph::PyDiGraph {
        graph: out_graph,
        cycle_state: algo::DfsSpace::default(),
        check_cycle: false,
        node_removed: false,
        multigraph: true,
    })
}
