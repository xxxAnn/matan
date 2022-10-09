use std::time::Duration;

use crate::{things::{exp::Expression, funcs::{RealRenderable, self, linear}}, obj::Object, alias::{DefaultColor, DefaultPoint}};

/// This is the only things the user should interact with

struct LayoutBuilder;

impl LayoutBuilder {
    fn two_dimension() -> TwoDimensionalBuilder {
        TwoDimensionalBuilder {}
    }
}

struct TwoDimensionalBuilder;

impl TwoDimensionalBuilder {
    fn real_real() -> RealNumberPairPlaneBuilder {
        RealNumberPairPlaneBuilder::new()
    }
}

struct RealNumberPairPlaneBuilder {
    exps: Vec<Expression>,
    objs: Vec<RealRenderable>
}

struct LinearFunctionBuilder {
    f: Vec<(u64, u64, linear::FunctionDescriptor)>
}

impl LinearFunctionBuilder {
    fn build(&self, screen_size: (u32, u32)) -> linear::Function {
        let mut n = linear::Function::new(screen_size);
        for (s, e, d) in &self.f {
            n.add_snapshot(*s, Duration::from_millis(e-s), Some(d.clone()));
        }
        n
    }
}

impl RealNumberPairPlaneBuilder {
    fn new() -> Self {
        Self {
            exps: vec![],
            objs: vec![]
        }
    }
    
    fn add_real_renderable(&mut self, r: RealRenderable) {
        self.objs.push(r);
    }

    pub fn add_linear_function<T: Fn(LinearFunctionBuilder) -> LinearFunctionBuilder>(&mut self, f: T) {
        todo!()
        //self.add_real_renderable(f().build())
    }
}