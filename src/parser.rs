use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::models::{Dimension, Node};

/// YAML frontmatter schema for a `.md` knowledge node file.
///
/// Example frontmatter:
/// ```yaml
/// ---
/// id: 550e8400-e29b-41d4-a716-446655440000
/// type: Concept
/// weight: 0.8
/// dimensions:
///   logical_next: ["550e8400-e29b-41d4-a716-446655440001"]
///   analogy_to:   ["550e8400-e29b-41d4-a716-446655440002"]
/// tags: "brain/cortex/logic"
/// updated: "2026-04-09"
/// ---
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Frontmatter {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub weight: Option<f64>,
    pub dimensions: Option<DimensionPointers>,
    pub tags: Option<String>,
    pub updated: Option<String>,
}

/// Dimension pointer lists embedded in the YAML frontmatter.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimensionPointers {
    pub logical_next: Option<Vec<String>>,
    pub historical_prev: Option<Vec<String>>,
    pub analogy_to: Option<Vec<String>>,
    pub contradicts: Option<Vec<String>>,
    pub refers_to: Option<Vec<String>>,
}

/// A parsed edge ready to be inserted into the [`KnowledgeGraph`].
pub struct ParsedEdge {
    /// UUID string of the source node (from the frontmatter `id` field).
    pub from_id: String,
    /// UUID string of the target node.
    pub to_id: String,
    /// Semantic relationship type.
    pub dimension: Dimension,
}

/// Result of parsing a single Markdown file.
pub struct ParsedNode {
    pub node: Node,
    pub edges: Vec<ParsedEdge>,
}

/// Parse a Markdown file that may contain YAML frontmatter delimited by `---`.
///
/// Returns `None` when the file does not contain a valid frontmatter block.
pub fn parse_markdown(source: &str) -> Option<ParsedNode> {
    let (frontmatter, body) = extract_frontmatter(source)?;
    let fm: Frontmatter = serde_yaml::from_str(frontmatter).ok()?;

    let title = body
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").trim().to_owned())
        .unwrap_or_default();

    let content = body.trim().to_owned();

    let mut node = Node::new(title, content);

    if let Some(ref id_str) = fm.id {
        if let Ok(uuid) = id_str.parse() {
            node.id = uuid;
        }
    }

    if let Some(w) = fm.weight {
        node.weight = w;
    }

    let mut metadata: HashMap<String, String> = HashMap::new();
    if let Some(ref t) = fm.node_type {
        metadata.insert("type".into(), t.clone());
    }
    if let Some(ref tags) = fm.tags {
        metadata.insert("tags".into(), tags.clone());
    }
    if let Some(ref updated) = fm.updated {
        metadata.insert("updated".into(), updated.clone());
    }
    node.metadata = metadata;

    let source_id = node.id.to_string();
    let mut edges: Vec<ParsedEdge> = Vec::new();

    if let Some(ref dims) = fm.dimensions {
        collect_edges(&source_id, &dims.logical_next, Dimension::LogicalNext, &mut edges);
        collect_edges(&source_id, &dims.historical_prev, Dimension::HistoricalOrigin, &mut edges);
        collect_edges(&source_id, &dims.analogy_to, Dimension::Analogy, &mut edges);
        collect_edges(&source_id, &dims.contradicts, Dimension::Contradiction, &mut edges);
        collect_edges(&source_id, &dims.refers_to, Dimension::RefersTo, &mut edges);
    }

    Some(ParsedNode { node, edges })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Split a Markdown source string into the raw frontmatter YAML and the body.
///
/// Returns `None` when there is no opening `---` delimiter.
fn extract_frontmatter(source: &str) -> Option<(&str, &str)> {
    let source = source.trim_start();
    if !source.starts_with("---") {
        return None;
    }
    let after_open = source.get(3..)?;
    // Find the closing `---`
    let close = after_open.find("\n---")?;
    let yaml = after_open[..close].trim();
    let body = after_open[close + 4..].trim_start_matches('\n');
    Some((yaml, body))
}

fn collect_edges(
    from_id: &str,
    targets: &Option<Vec<String>>,
    dimension: Dimension,
    out: &mut Vec<ParsedEdge>,
) {
    if let Some(ids) = targets {
        for to_id in ids {
            out.push(ParsedEdge {
                from_id: from_id.to_owned(),
                to_id: to_id.clone(),
                dimension: dimension.clone(),
            });
        }
    }
}
