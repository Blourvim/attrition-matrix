/// To html method for this endpoint is moved to the diff engine,
///  and left there since i releazied it was easier to skip.
/// Not a very good decision in hindsight
use actix_web::{FromRequest, HttpRequest};
use serde::Deserialize;

pub struct AttritionMatrixResponse {
    pub matrix: Vec<Vec<Sdk>>,
}

pub struct Sdk {
    pub id: i64,
    pub amount: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AttritionMatrixQuery {
    pub sdks: Vec<i64>,
}

// https://github.com/actix/actix-web/issues/786
// since serde_urlencoded doesn't support vectors, I am going to write my own extractor
// https://leapcell.io/blog/crafting-custom-extractors-in-axum-and-actix-web

impl FromRequest for AttritionMatrixQuery {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, actix_web::Error>>;

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let sdks: Vec<i64> = vector_query_parser(req.query_string()).unwrap();
        std::future::ready(Ok(AttritionMatrixQuery { sdks }))
    }
}

fn vector_query_parser(query: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    // todo: This can be more robust, with better error handling
    let query = query.strip_prefix('?').unwrap_or(query);
    let split_query: Vec<i64> = query
        .split("&")
        .filter(|s| s.starts_with("sdks="))
        .filter(|s| s.strip_prefix("sdks=").unwrap().parse::<i64>().is_ok())
        .map(|s| s.strip_prefix("sdks=").unwrap().parse::<i64>().unwrap())
        .collect();

    return Ok(split_query);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_happy_parse() {
        let string = "sdks=1&sdks=2";
        let parsed = vector_query_parser(string);

        let valid_vec = vec![1, 2];
        assert_eq!(parsed.unwrap(), valid_vec);
    }
    #[test]

    fn test_sad_parse() {
        let string = "sdkss=1&sdks=2";
        let parsed = vector_query_parser(string).unwrap();
        let invalid_vec = vec![1, 2];
        let valid_vec = vec![2];

        assert_ne!(parsed.clone(), invalid_vec);
        assert_eq!(parsed.clone(), valid_vec);
    }
}
