use self::filters::FilterModule;

pub mod filters;

#[derive(Debug, impl_new::New,Default)]
pub struct PreRoutingModules {
    pub filter: FilterModule,
}
