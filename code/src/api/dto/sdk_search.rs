use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct SdkSearchQuery {
    pub search: String,
}

impl Sdk {
    pub fn new(name: String, id: i64, logo_url: String) -> Self {
        Self { name, id, logo_url }
    }
    pub fn to_html(&self) -> String {
        format!(
            "<option 
            id={},
        hx-get=\"/api/matrix\"
        hx-trigger=\"click\" 
        hx-swap=\"innerHTML\" 
        hx-target=\"#matrix\" 
        hx-vals='js:{{ sdks:[1,2,3,4] }}' 
        value={}>{}</option>",
            self.id, self.id, self.name
        )
    }
}
impl SdkSearchResponse {
    pub fn new(search: String) -> Self {
        let sdks: Vec<Sdk> = (0..2)
            .map(|f| {
                Sdk::new(
                    search.clone(),
                    f,
                    "https://picsum.photos/200/300".to_string(),
                )
            })
            .collect();

        Self { sdks }
    }
    pub fn to_html(&self) -> String {
        self.sdks
            .iter()
            .map(|sdk| sdk.to_html())
            .collect::<String>()
    }
}
