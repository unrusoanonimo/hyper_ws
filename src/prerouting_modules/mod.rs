use self::modifiers::ModifierModule;

mod modifiers;

#[derive(Debug, Default, impl_new::New)]
pub struct PreroutingModules {
    pub modifiers: ModifierModule,
}
