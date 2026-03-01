use serde::Serialize;

#[derive(Serialize)]
pub struct SdkSearchResponse {
    pub sdks: Vec<Sdk>,
}

#[derive(Serialize)]
pub struct Sdk {
    pub name: String,
    pub id: i64,
    pub logo_url: String,
}
