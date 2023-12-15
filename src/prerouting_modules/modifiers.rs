use derivative::Derivative;
use http::Response;
use hyper::Body;

pub trait Modifier: Send + Sync + 'static {
    fn call(&self, res: &mut Response<Body>);
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ModifierModule {
    #[derivative(Debug = "ignore")]
    values: Vec<Box<dyn Modifier>>,
}

impl ModifierModule {
    pub fn new(values: impl Into<Vec<Box<dyn Modifier>>>) -> Self {
        Self {
            values: values.into(),
        }
    }
    pub fn add(&mut self, m: impl Modifier) {
        self.values.push(Box::new(m));
    }
    pub fn call(&self, res: &mut Response<Body>) {
        self.values.iter().for_each(|m| m.call(res));
    }
}
impl Default for ModifierModule {
    fn default() -> Self {
        Self::new([])
    }
}

struct SesionModifier {}
impl Modifier for SesionModifier {
    fn call(&self, res: &mut Response<Body>) {
        
    }
}
