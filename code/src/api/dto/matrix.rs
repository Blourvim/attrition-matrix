use serde::{Deserialize, Serialize};

pub struct AttritionMatrixResponse {
    pub matrix: Vec<Vec<Sdk>>,
}

pub struct Sdk {
    pub id: i64,
    pub amount: i64,
    pub name: String,
}

impl Sdk {
    pub fn new() -> Self {
        Self {
            amount: 0,
            id: 0,
            name: "".to_string(),
        }
    }

    pub fn to_html(&self) -> String {
        "<td 
            id=1
            hx-get=\"/api/example_apps\"
            hx-trigger=\"mouseenter\"
            hx-swap=\"innerHTML\" 
            hx-target=\"this\"
            hx-vals='js:{ sdk_id: this.id }'
            class=\"table-cell\">&nbsp;
        
        </td>"
            .to_string()
    }
}

impl AttritionMatrixResponse {
    pub fn new(amount: usize) -> Self {
        Self {
            matrix: (0..amount)
                .map(|y| (0..amount).map(|x| Sdk::new()).collect())
                .collect(),
        }
    }

    pub fn to_html(&self) -> String {
        self.matrix
            .iter()
            .map(|row| {
                let mut html_response = "<tr>".to_string();
                html_response.push_str(&row.iter().map(|sdk| sdk.to_html()).collect::<String>());
                html_response.push_str("</tr>");
                html_response
            })
            .collect::<String>()
    }
}
#[derive(Deserialize)]
pub struct AttritionMatrixQuery {
    pub sdks: Vec<i64>,
}
