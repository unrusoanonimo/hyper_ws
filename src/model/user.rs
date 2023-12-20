use optfield::optfield;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[optfield(pub PartialUser, rewrap, attrs, merge_fn = pub merge, from )]
#[derive(Serialize, Deserialize, Debug, Clone, Default, impl_new::New)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub deleted: bool,
    pub password: String,
}
