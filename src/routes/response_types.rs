use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CountResponse {
    pub count: usize,
}
