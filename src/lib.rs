//! computational_node allow to create direct acyclic graph of operations on input values with caching inside nodes.
//! Graph does not create common storage for the node, the graph consists of individual nodes and links for computation and cache invalidation.
//! # Example:
//! ```rust
//! # use computational_graph::*;
//! # // round to decimal digits
//! # fn round(x: f32, precision: u32) -> f32 {
//! #   let m = 10i32.pow(precision) as f32;
//! #   (x * m).round() / m
//! # }
//! // x1, x2, x3 are input nodes of the computational graph:
//! let x1 = create_input("x1");
//! let x2 = create_input("x2");
//! let x3 = create_input("x3");
//! // graph variable is the output node of the graph:
//! let graph = add(
//!     x1.clone(),
//!     mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
//! );
//! x1.set(1f32);
//! x2.set(2f32);
//! x3.set(3f32);
//! let mut result = graph.compute();
//! result = round(result, 5);
//! println!("Graph output = {}", result);
//! assert_eq!(round(result, 5), -0.32727);
//! x1.set(2f32);
//! x2.set(3f32);
//! x3.set(4f32);
//! result = graph.compute();
//! result = round(result, 5);
//! println!("Graph output = {}", result);
//! assert_eq!(round(result, 5), -0.56656);
//! ```

pub mod node;
pub mod cache;
pub mod utils;
pub mod operations;

pub use utils::*;
pub use node::Node;

#[cfg(test)]
mod tests {
    use super::*;

    // round to decimal digits
    fn round(x: f32, precision: u32) -> f32 {
        let m = 10i32.pow(precision) as f32;
        (x * m).round() / m
    }

    #[test]
    fn test_input_ref() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        x1.set(1.0);
        let graph = add(x1.clone(), x2.clone());
        x2.set(2.0);
        let res = graph.compute();
        assert_eq!(res, 3.0);
    }

    #[test]
    fn test_input_invalidation() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let graph = add(x1.clone(), x2.clone());
        x1.set(1.0);
        x2.set(2.0);
        let res = graph.compute();
        assert_eq!(res, 3.0);
        x2.set(-1.0);
        let res = graph.compute();
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_add_reuse() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let graph = add(x1.clone(), x2.clone());
        x1.set(1.0);
        x2.set(2.0);
        let res = graph.compute();
        assert_eq!(res, 3.0);
        let graph = add(graph.clone(), x1.clone());
        let res = graph.compute();
        assert_eq!(res, 4.0);
        let graph = add(graph.clone(), graph.clone());
        let res = graph.compute();
        assert_eq!(res, 8.0);
        x1.set(-2.0);
        let res = graph.compute();
        assert_eq!(res, -4.0);
    }

    #[test]
    fn test_task_reference() {
        // x1, x2, x3 are input nodes of the computational graph:
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let x3 = create_input("x3");
        // graph variable is the output node of the graph:
        let graph = add(
            x1.clone(),
            mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
        );
        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);
        let mut result = graph.compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.32727);
        x1.set(2f32);
        x2.set(3f32);
        x3.set(4f32);
        result = graph.compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.56656);
    }
}
