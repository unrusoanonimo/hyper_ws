use derivative::Derivative;
use http::Response;
use hyper::Body;

use crate::{util::ExtendedRequest, ModulesSendable};

pub trait Modifier: Send + Sync + 'static {
    fn call(&self, req: &mut ExtendedRequest, res: &mut Response<Body>, modules: ModulesSendable);
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ModifierModule {
    #[derivative(Debug = "ignore")]
    values: Vec<Box<dyn Modifier>>,
}

impl ModifierModule {
    pub fn new(values: Vec<Box<dyn Modifier>>) -> Self {
        Self {
            values: values.into(),
        }
    }
    pub fn add(&mut self, m: impl Modifier) {
        self.values.push(Box::new(m));
    }
    pub fn call(
        &self,
        req: &mut ExtendedRequest,
        res: &mut Response<Body>,
        modules: ModulesSendable,
    ) {
        self.values
            .iter()
            .for_each(|m| m.call(req, res, modules.clone()));
    }
}
impl Default for ModifierModule {
    fn default() -> Self {
        Self::new(vec![])
    }
}
