use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CredentialsLogin {
    pub email_address: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct CredentialsRegister {
    pub firstname: String,
    pub lastname: String,
    pub email_address: String,
    pub password: String,
}
