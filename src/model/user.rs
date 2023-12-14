use uuid::Uuid;

struct User {
    id: Uuid,
    username: String,
    deleted: bool,
}
