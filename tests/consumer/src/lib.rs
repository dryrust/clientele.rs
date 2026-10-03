//! Downstream compilation tests for the actual public documentation examples.

#[doc = include_str!(concat!(env!("OUT_DIR"), "/examples.md"))]
pub struct Examples;
