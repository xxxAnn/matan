use std::time::Duration;

use crate::{things::{exp::Expression, funcs::{RealRenderable, linear}, number_planes::real_pair::RealNumberPairPlane}, obj::{Object, RenderCommand, CommandGrid}, alias::{DefaultColor, DefaultPoint}};

/// This is the only things the user should interact with

pub struct LayoutBuilder;

impl LayoutBuilder {
    pub fn two_dimension() -> TwoDimensionalBuilder {
        TwoDimensionalBuilder {}
    }
}

pub struct TwoDimensionalBuilder;

impl TwoDimensionalBuilder {
    pub fn real_real(&self, screen_size: (u32, u32)) -> RealNumberPairPlaneBuilder {
        RealNumberPairPlaneBuilder::new(screen_size)
    }
}

pub struct RealNumberPairPlaneBuilder {
    exps: Vec<Expression>,
    objs: Vec<((u128, u128), RenderCommand<DefaultColor, DefaultPoint>)>,
    screen_size: (u32, u32)
}

pub struct LinearFunctionBuilder {
    f: Vec<(u64, u64, linear::FunctionDescriptor)>
}

impl LinearFunctionBuilder {
    fn new() -> Self {
        Self { 
            f: vec![]
        }
    }

    pub fn add_snapshot_from_angular_form(mut self, from: u64, to: u64, ang: f32, intercept: f32, width: f32) -> Self {
        self.f.push((from, to, linear::FunctionDescriptor::Angle(linear::AngularDescriptor::new(ang, intercept, width))));
        self
    }

    pub fn add_snapshot_from_slope_form(mut self, from: u64, to: u64, slope: f32, intercept: f32, width: f32) -> Self {
        self.f.push((from, to, linear::FunctionDescriptor::Standard(linear::SlopeDescriptor::new(slope, intercept, width))));
        self
    }

    pub fn add_snapshots_from_slope_form(mut self, from: u64, to: u64, parts: &[(f32, f32, f32)]) -> Self {
        let dv = (((to-from) as usize)/parts.len()) as u64;
        //println!("Timestamps:");
        for i in 0..parts.len() {
            let cs = (from+dv*((i) as u64), from+dv*((i+1) as u64));
            //println!("{:?}", &cs);
            let n = parts[i];
            self.f.push((cs.0, cs.1, linear::FunctionDescriptor::Angle(linear::AngularDescriptor::new(n.0, n.1, n.2))));
        }
        self
    }

    pub fn add_transition_from_slope_form(self, from: u64, to: u64, start: (f32, f32, f32), end: (f32, f32, f32), divs: usize) -> Self {
        let k = divs as f32;
        let n = ((end.0-start.0)/k, (end.1-start.1)/k, (end.2-start.2)/k);
        let mut  parts = Vec::new();
        for i in 0..(divs+1) {
            let m = i as f32;
            parts.push((start.0 + n.0*m, start.1 + n.1*m, start.2 + n.2*m));
        }
        self.add_snapshots_from_slope_form(from, to, &parts)
    }

    fn build(&self, screen_size: (u32, u32)) -> (Vec<(u64, u64)>, linear::Function) {
        let mut n = linear::Function::new(screen_size);
        for (s, e, d) in &self.f {
            n.add_snapshot(*s, Duration::from_millis(e-s), Some(d.clone()));
        }
        (self.f.iter().map(|x| (x.0, x.1)).collect(), n)
    }
}

impl RealNumberPairPlaneBuilder {
    fn new(screen_size: (u32, u32)) -> Self {
        Self {
            exps: vec![],
            objs: vec![],
            screen_size
        }
    }
    
    fn add_real_renderable(&mut self, s: (u128, u128), r: RenderCommand<DefaultColor, DefaultPoint>) {
        self.objs.push((s, r));
    }

    pub fn add_linear_function<T: Fn(LinearFunctionBuilder) -> LinearFunctionBuilder>(mut self, f: T) -> Self {
        let x = f(LinearFunctionBuilder::new());
        let t = x.build(self.screen_size);
        for x in t.0.iter() {
            self.add_real_renderable(
                (u128::from(x.0), u128::from(x.1)), 
                t.1.render(u128::from((x.0+x.1)/2))
            )
        }
        self
    }

    fn build(self) -> RealNumberPairPlane {
        let mut r = RealNumberPairPlane::new();
        let rr = RealRenderable::new(self.objs);
        r.add_object(rr);
        for e in self.exps.into_iter() {
            r.add_text(e);
        }
        r
    }

    pub fn run(self, title: &str) -> Result<(), String> {
        let size = self.screen_size;
        let mut x = self.build();
        x.start(title, size.0, size.1)?;
        Ok(())
    }
}