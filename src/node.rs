//! Basic types to create computational graph with caching.

use std::rc::{Rc, Weak};
use std::cell::{RefCell, Cell};

/// Node trait represent a compute graph node that can return a (cached) value, get call for invalidation
/// and get link to another node dependent on the current and so its cache must be invaludated
/// once the value of the current node changes.
/// Object-safe for the purpose of building the computatin grapth using Rc to dyn objects.
/// 
/// # Example:
/// ```
/// # use computational_graph::*;
/// # // round to decimal digits
/// # fn round(x: f32, precision: u32) -> f32 {
/// #   let m = 10i32.pow(precision) as f32;
/// #   (x * m).round() / m
/// # }
/// // x1, x2, x3 are input nodes of the computational graph:
/// let x1 = create_input("x1");
/// let x2 = create_input("x2");
/// let x3 = create_input("x3");
/// // graph variable is the output node of the graph:
/// let graph = add(
///     x1.clone(),
///     mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
/// );
/// x1.set(1f32);
/// x2.set(2f32);
/// x3.set(3f32);
/// let mut result = graph.compute();
/// result = round(result, 5);
/// println!("Graph output = {}", result);
/// assert_eq!(round(result, 5), -0.32727);
/// x1.set(2f32);
/// x2.set(3f32);
/// x3.set(4f32);
/// result = graph.compute();
/// result = round(result, 5);
/// println!("Graph output = {}", result);
/// assert_eq!(round(result, 5), -0.56656);
/// ```
/// 

pub trait Node {
    type Output;

    /// Provides the value of the node, that can be quickly retrieved from the cache, 
    /// or computations of unknown complexity will be performed
    fn compute(&self) -> Self::Output;
    /// Invalidate the cache of the current node and the dependent nodes.
    fn invalidate(&self);
    /// Add some node n to the list of the nodes that are dependent of the value of this node.
    fn add_dependent(&self, n: Rc<dyn Node<Output = Self::Output>>); 
}

/// Dependencies contain links to the dependent nodes that must be invalidated and recomputed once the value
/// of the current node changes. 
#[derive(Default)]
pub(crate) struct Dependencies<T> {
    vec: RefCell<Vec<Weak<dyn Node<Output = T>>>>
}

impl<T> Dependencies<T> {
    pub(crate) fn add(&self, n: Rc<dyn Node<Output = T>>) {
        // Rc are downgraded to Weak to prevent the occurrence of cyclic dependencies.
        self.vec.borrow_mut().push(Rc::downgrade(&n));
    }

    pub(crate) fn invalidate(&self) {
        for d in self.vec.borrow().iter() {
            d.upgrade().map(|x| x.invalidate());
        }
    }

}

/// Input node present some f32 input value. This node stores a list of dependent nodes `dep`
/// and invalidates their caches when the input values is changed.
pub struct Input<'a> {
    _name: &'a str,
    value: Cell<f32>,
    dep: Dependencies<f32>
}

impl<'a> Input<'a> {
    pub fn new(_name: &'a str) -> Input<'a>{
        Input { _name, value: Default::default(), dep: Default::default() }
    }

    /// Set new value `x` and require invalidation of the caches of the dependent nodes.
    pub fn set(&self, x: f32) {
        self.invalidate();
        self.value.set(x);
    }
}

impl<'a> Node for Input<'a> {
    type Output = f32;

    fn compute(&self) -> Self::Output {
        self.value.get()
    }

    /// Require invalidation of the dependent nodes.
    fn invalidate(&self) {
        self.dep.invalidate();
    }

    fn add_dependent(&self, n: Rc<dyn Node<Output = Self::Output>>) {
        self.dep.add(n);
    }
}