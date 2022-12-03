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

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::create_input;
    use super::*;

    #[test]
    fn test_unary_op() {
        let flag = Rc::new(Cell::new(false));
        let res = {
            let flag = flag.clone();
            let input = create_input("foo");   
            input.set(3.3);
            let f = move |x| { flag.set(true); x };  
            Unary::new(input, f).compute()   
        };
        assert_eq!(res, 3.3);
        assert!(flag.get())
    }

    #[test]
    fn test_binary_op() {
        let flag = Rc::new(Cell::new(false));
        let res = {
            let flag = flag.clone();
            let input1 = create_input("foo");
            let input2 = create_input("bas");
            input1.set(3.3);
            input2.set(5.0);
            let f = move |x,y| { flag.set(true); x+y };
            Binary::new(input1, input2, f).compute()
        };
        assert_eq!(res, 8.3);
        assert!(flag.get())
    }

    #[test]
    fn test_unrary_op_caching() {
        let x = Rc::new(Cell::new(0));
        let node = {
            let x = x.clone();
            let input = create_input("foo");
            let f = move |_| {
                let v = x.get();
                x.set(v+1); 
                3.3
            };
            Unary::new(input, f)
        };
        assert_eq!(node.compute(), 3.3);
        assert_eq!(node.compute(), 3.3);
        assert_eq!(x.get(), 1);
        node.invalidate();
        assert_eq!(node.compute(), 3.3);
        assert_eq!(x.get(), 2);
    }

    #[test]
    fn test_binary_op_caching() {
        let flag = Rc::new(Cell::new(0));
        let node = {
            let flag = flag.clone();
            let input1 = create_input("foo");
            let input2 = create_input("bar");
            input1.set(3.3);
            input2.set(5.0);
            let f = move |x,y| {
                let v = flag.get();
                flag.set(v+1);
                x+y
            };  
            Binary::new(input1, input2, f)
        };
        assert_eq!(flag.get(), 0);
        assert_eq!(node.compute(), 8.3);
        assert_eq!(node.compute(), 8.3);
        assert_eq!(flag.get(), 1);
        node.invalidate();
        assert_eq!(node.compute(), 8.3);
        assert_eq!(flag.get(), 2);
    }
}