pub mod nodes;
use nodes::boolean::{And, Not};
pub mod graph;
pub mod node;
use node::{InputId, Node, NodeId, OutputId};
pub mod connection;
use connection::{ConnType, Connection};

pub fn not_and() {
    let a = (0..).map(|int| int % 3 == 0);
    let b = (0..).map(|int| int % 5 == 0);
    let and = And::new(Box::new(a), Box::new(b));
    let mut not_and = Not::new(Box::new(and));

    for i in 0..100 {
        println!("{i} !(A & B) is {:?}", not_and.next());
    }
}

fn main() {}
