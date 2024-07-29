// pub mod and;
// mod connection;
// mod graph;
// mod node;
// pub mod nodes;

// pub use connection::{ConnType, Connection};
// pub use graph::{Graph, GraphError};
// pub use node::{InputId, Node, NodeId, OutputId};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn multi_delay_processing() {
//         let mut graph: Graph<Box<dyn Node>> = Graph::new();
//         let del0 = graph.add_node(Box::from(nodes::Delay::new()));
//         let del1 = graph.add_node(Box::from(nodes::Delay::new()));
//         let del2 = graph.add_node(Box::from(nodes::Delay::new()));
//         graph.add_connection(Connection::new(ConnType::Pipe, del0, OutputId(0), del1, InputId(0))).unwrap();
//         graph.add_connection(Connection::new(ConnType::Pipe, del1, OutputId(0), del2, InputId(0))).unwrap();
//         graph.get_node_mut(del0).unwrap().set_input(InputId(0), 1.0);

//         assert_eq!(graph.get_node(del0).unwrap().get_output(OutputId(0)), 0.0);
//         assert_eq!(graph.get_node(del1).unwrap().get_output(OutputId(0)), 0.0);
//         assert_eq!(graph.get_node(del2).unwrap().get_output(OutputId(0)), 0.0);
//         graph.process();
//         assert_eq!(graph.get_node(del0).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del1).unwrap().get_output(OutputId(0)), 0.0);
//         assert_eq!(graph.get_node(del2).unwrap().get_output(OutputId(0)), 0.0);
//         graph.process();
//         assert_eq!(graph.get_node(del0).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del1).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del2).unwrap().get_output(OutputId(0)), 0.0);
//         graph.process();
//         assert_eq!(graph.get_node(del0).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del1).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del2).unwrap().get_output(OutputId(0)), 1.0);
//         graph.process();
//         assert_eq!(graph.get_node(del0).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del1).unwrap().get_output(OutputId(0)), 1.0);
//         assert_eq!(graph.get_node(del2).unwrap().get_output(OutputId(0)), 1.0);
//     }

//     fn a() {
//         let A = (0..).map(|int| int % 3 == 0);
//         let B = (3..).map(|int| int % 5 == 0);
//         let mut and = and::And::new(Box::new(A), Box::new(B));
//         loop {
//             println!("A & B is {:?}", and.next());
//         }
//     }
// }
