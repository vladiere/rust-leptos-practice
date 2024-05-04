use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ErrorStatus {
    pub message: String,
    pub status: i32,
}
