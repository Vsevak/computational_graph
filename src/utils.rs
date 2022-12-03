//! Includes some functions to create computational graph with common math operations.

use crate::node::{Input, Node};
use crate::operations::{Binary, Unary};

use std::rc::Rc;

/// Creates input node of the compute graph with a given name
pub fn create_input<'a>(name: &'a str) -> Rc<Input<'a>>{
    Rc::new( Input::new(name) )
}

/// Creates summation node that add outputs of two given nodes and cache it.
pub fn add(x: Rc<dyn Node<Output = f32>>, y: Rc<dyn Node<Output = f32>>) -> Rc<dyn Node<Output = f32>> {
    Binary::new(x, y, |x,y| x+y)
}

/// Creates multiplication node that multiply outputs of two given nodes and cache it.
pub fn mul(x: Rc<dyn Node<Output = f32>>, y: Rc<dyn Node<Output = f32>>) -> Rc<dyn Node<Output = f32>> {
    Binary::new(x, y, |x,y| x*y)
}

/// Creates new node that compute trigonometric sinus of a value of a given nodes and cache it.
pub fn sin(x: Rc<dyn Node<Output = f32>>) -> Rc<dyn Node<Output = f32>> {
    Unary::new(x, |x| x.sin())
}

/// Creates new node that apply power function with a given exponent e to the value of some node.
pub fn pow_f32(x: Rc<dyn Node<Output = f32>>, e: f32) -> Rc<dyn Node<Output = f32>> {
    Unary::new(x, move |x| f32::powf(x, e))
}
