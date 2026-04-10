use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an atomic knowledge unit in the Neural-Wiki graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this knowledge node.
    pub id: Uuid,
    /// Short human-readable title.
    pub title: String,
    /// The main content / body of this knowledge unit.
    pub content: String,
    /// Activation weight in the range [0.0, 1.0]; higher means more frequently activated.
    pub weight: f64,
    /// Arbitrary key-value metadata (e.g. tags, node type, last-updated date).
    pub metadata: HashMap<String, String>,
}

impl Node {
    /// Create a new node with the given title and content.
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            content: content.into(),
            weight: 0.5,
            metadata: HashMap::new(),
        }
    }
}

/// Describes the semantic relationship type carried by an edge in the knowledge graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    /// The target node is the logical next step in a reasoning chain.
    LogicalNext,
    /// The target node is the historical origin / predecessor.
    HistoricalOrigin,
    /// The target node is an analogy (cross-domain mapping).
    Analogy,
    /// The target node contradicts or is a counter-example of this node.
    Contradiction,
    /// A generic reference or citation relationship.
    RefersTo,
}

/// The central knowledge graph.
///
/// Nodes hold [`Node`] data; edges are labelled with a [`Dimension`] describing
/// the semantic relationship between the two knowledge units.
pub type WikiGraph = DiGraph<Node, Dimension>;

/// Convenience wrapper that pairs the raw petgraph graph with an index map so
/// that nodes can be looked up by their [`Uuid`].
pub struct KnowledgeGraph {
    pub graph: WikiGraph,
    /// Maps a node's UUID to its petgraph [`NodeIndex`].
    pub index: HashMap<Uuid, NodeIndex>,
}

impl KnowledgeGraph {
    /// Create an empty knowledge graph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index: HashMap::new(),
        }
    }

    /// Insert a [`Node`] and return its [`NodeIndex`].
    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        let id = node.id;
        let idx = self.graph.add_node(node);
        self.index.insert(id, idx);
        idx
    }

    /// Add a directed edge between two nodes identified by their UUIDs.
    ///
    /// Returns `Some(edge_index)` on success, or `None` if either UUID is not
    /// present in the graph.
    pub fn add_edge(
        &mut self,
        from: Uuid,
        to: Uuid,
        dimension: Dimension,
    ) -> Option<petgraph::graph::EdgeIndex> {
        let &from_idx = self.index.get(&from)?;
        let &to_idx = self.index.get(&to)?;
        Some(self.graph.add_edge(from_idx, to_idx, dimension))
    }

    /// Look up a node by UUID.
    pub fn get_node(&self, id: &Uuid) -> Option<&Node> {
        let idx = self.index.get(id)?;
        self.graph.node_weight(*idx)
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}
