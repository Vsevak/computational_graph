//! Provides some operations as building blocks to create computational graph.

use crate::node::{Node, Dependencies};
use crate::cache::Cache;

use std::rc::Rc;

/// Binary type of Node takes two inputs nodes (`x` and `y`) and operation (`op`) on them. 
/// This type provides caching of the computations and invaludation of its cache and dependent nodes.
pub struct Binary<T: Fn(f32,f32) -> f32> {
    x: Rc<dyn Node<Output = f32>>,
    y: Rc<dyn Node<Output = f32>>,
    op: T,
    cached: Cache<f32>,
    dep: Dependencies<f32>
}

impl<T: Fn(f32,f32) -> f32 + 'static> Binary<T> {
    pub fn new(x: Rc<dyn Node<Output = f32>>, y: Rc<dyn Node<Output = f32>>, op: T) -> Rc<Self> {
        // Create new binary node
        let tmp = Rc::new( 
            Self { x: x.clone(), y: y.clone(), op, dep: Default::default(), cached: Cache::new() } 
        );
        // Add a new node to the lists of the input nodes
        x.add_dependent(tmp.clone());
        y.add_dependent(tmp.clone());
        tmp
    }
}

impl<T: Fn(f32,f32) -> f32> Node for Binary<T> {
    type Output = f32;

    fn compute(&self) -> f32 {
        // Get cached value or compute the result
        self.cached.get_or_else(|| (self.op)(self.x.compute(), self.y.compute()))
    }

    fn invalidate(&self) {
        self.cached.invalidate();
        self.dep.invalidate();
    }

    fn add_dependent(&self, n: Rc<dyn Node<Output = f32>>) {
        self.dep.add(n);
    }
}

/// Unary type of Node takes a single inputs nodes (`x`) and operation (`op`) as Fn. This type provides caching
/// of the computations and invaludation of its cache and dependent nodes.
pub struct Unary<T: Fn(f32) -> f32> {
    x: Rc<dyn Node<Output = f32>>,
    op: T,
    cached: Cache<f32>,
    dep: Dependencies<f32>
}

impl<T: Fn(f32) -> f32 + 'static> Unary<T> {
    pub fn new(x: Rc<dyn Node<Output = f32>>, op: T) -> Rc<Self> {
        // Create new unary node
        let tmp = Rc::new( 
            Self { x: x.clone(), op, dep: Default::default(), cached: Cache::new() } 
        );
        // Add the new node to the list of dependent nodes.
        x.add_dependent(tmp.clone());
        tmp
    }
}

impl<T: Fn(f32) -> f32> Node for Unary<T> {
    type Output = f32;

    /// Get cached value or apply the operation to the input.
    fn compute(&self) -> f32 {
        self.cached.get_or_else(|| (self.op)(self.x.compute()) )
    }

    /// Invalidate its own cache and then invalidate the dependent nodes.
    fn invalidate(&self) {
        self.cached.invalidate();
        self.dep.invalidate();
    }

    fn add_dependent(&self, n: Rc<dyn Node<Output = f32>>) {
        self.dep.add(n);
    }
}
