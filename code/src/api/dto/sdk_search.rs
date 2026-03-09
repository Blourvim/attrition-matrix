use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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
        onclick=\"add_sdk({})\"
        id={},
        hx-get=\"/api/matrix\"
        hx-trigger=\"click\" 
        hx-swap=\"innerHTML\" 
        hx-target=\"#matrix-area\" 
        hx-vals='js:{{ sdks:sdks }}' 
        value={}>{}
        </option>",
            self.id, self.id, self.id, self.name,
        )
    }
}
impl SdkSearchResponse {
    pub async fn new(search: String, db: &sea_orm::DatabaseConnection) -> Self {
        let sdks_response = entity::sdk::Entity::find()
            .filter(entity::sdk::Column::Name.contains(search))
            .all(db)
            .await;
        if let Ok(sdks) = sdks_response {
            let sdks: Vec<Sdk> = sdks
                .iter()
                .map(|f| {
                    Sdk::new(
                        f.name.clone().unwrap(),
                        f.id,
                        f.url.clone().unwrap_or_default(),
                    )
                })
                .collect();
            return SdkSearchResponse { sdks };
        } else {
            return SdkSearchResponse { sdks: vec![] };
        }
    }
    pub fn to_html(&self) -> String {
        self.sdks
            .iter()
            .map(|sdk| sdk.to_html())
            .collect::<String>()
    }
}
