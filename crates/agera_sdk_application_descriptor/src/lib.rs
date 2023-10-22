use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApplicationDescriptor {
    pub id: String,
}