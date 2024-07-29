use crate::{InputId, NodeId, OutputId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnType {
    Pipe,
    Feedback(Option<f64>), // the last value that was passed
}
/// Graph edge between source node output and target node input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Connection {
    pub conn_type: ConnType,
    pub source_node: NodeId,
    pub source_output: OutputId,
    pub target_input: InputId,
    pub target_node: NodeId,
}
impl Connection {
    /// Creates new connection.
    pub fn new(
        conn_type: ConnType,
        source_node: NodeId,
        source_output: OutputId,
        target_node: NodeId,
        target_input: InputId,
    ) -> Self {
        Connection { conn_type, source_node, source_output, target_input, target_node }
    }
}
