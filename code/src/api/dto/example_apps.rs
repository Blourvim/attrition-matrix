// This code right now, is non-functional

use serde::Deserialize;

pub struct ExampleApps {
    apps: Vec<App>,
}

pub struct App {
    id: i64,
    name: String,
    logo: String,
}

impl ExampleApps {
    //mock for now
    pub fn new(sdk_id: i64) -> Self {
        ExampleApps {
            apps: (0..sdk_id)
                .map(|i| App {
                    name: "test".to_string(),
                    id: i as i64,
                    logo: "https://picsum.photos/200".to_string(),
                })
                .collect(),
        }
    }
    pub fn to_html(&self) -> String {
        let mut html_response = "<div>".to_string();
        self.apps
            .iter()
            .for_each(|app| html_response.push_str(&format!("<image src=\"{}\">", app.logo)));
        html_response.push_str("</div>");
        html_response
    }
}

#[derive(Deserialize)]
pub struct ExampleAppsQuery {
    pub sdk_id: i64,
}
