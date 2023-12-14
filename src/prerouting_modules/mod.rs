use http::Response;
use hyper::Body;

use crate::{
    util::{AppError, ExtendedRequest},
    ModulesSendable,
};

use self::filters::FilterModule;

pub mod filters;

#[derive(Debug, impl_new::New, Default)]
pub struct PreroutingModules {
    pub filter: FilterModule,
}
impl PreroutingModules {
    pub fn evaluate_modules(
        &self,
        req: &ExtendedRequest,
        modules: &mut ModulesSendable,
    ) -> PreroutingResolution {
        self.filter
            .evaluate_filters(req, modules)
            .unwrap_or_else(|e| PreroutingResolution::Return(e.into()))
    }
}

pub enum PreroutingResolution {
    Return(Response<Body>),
    Modify(Vec<Box<dyn FnOnce(&mut Response<Body>) -> Result<(), AppError>>>),
}
