use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AttritionMatrixResponse {}

#[derive(Deserialize)]
pub struct AttritionMatrixQuery {
    pub sdks: Vec<i64>,
}

pub struct Sdk {
    pub id: i64,
    pub amount: i64,
    pub name: String,
}
