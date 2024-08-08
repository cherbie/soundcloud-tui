use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_health).service(get_oauth2_redirect))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[actix_web::get("/health")]
async fn get_health(_req: HttpRequest) -> &'static str {
    "ok!"
}

#[actix_web::get("/oauth2/redirect")]
async fn get_oauth2_redirect(req: HttpRequest) -> HttpResponse {
    let uri_query_string = req.query_string();

    let params_map = match parse_query_string(uri_query_string) {
        Ok(params_map) => params_map,
        Err(err) => {
            eprintln!("Error parsing query string: {}", err);
            return HttpResponse::BadRequest().finish();
        }
    };
    if let Some(code) = params_map.get("code") {
        println!("code: {}", code);
    } else {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}

fn parse_query_string(
    query_string: &str,
) -> Result<std::collections::HashMap<String, String>, String> {
    query_string
        .split('&')
        .map(|pair| {
            let split: Vec<&str> = pair.split('=').filter(|s| s.len() > 0).collect();
            if split.len() == 0 {
                return Ok((None, None));
            } else if split.len() != 2 {
                return Err(format!("Invalid query string param ({})", pair));
            }

            Ok((
                split.get(0).and_then(|v| Some(v.to_string())),
                split.get(1).and_then(|v| Some(v.to_string())),
            ))
        })
        .filter_map(|pair| match pair {
            Ok((Some(key), Some(value))) => Some(Ok((key, value))),
            Ok((None, _)) => None,
            Err(err) => Some(Err(err)),
            _ => Some(Err(format!("No key-value pair found"))),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;

    #[test]
    fn test_query_string_singular() {
        let mock_value: String = (1..11).map(|_| rand::random::<char>()).collect();
        let query_string = format!("mockKey={}", mock_value);
        let params_map = parse_query_string(&query_string);
        assert!(params_map.is_ok());
        assert_eq!(params_map.unwrap().get("mockKey"), Some(&mock_value));
    }

    #[test]
    fn test_query_string_multiple() {
        let mock_value: String = (1..11).map(|_| rand::random::<char>()).collect();
        let mock_key: String = (1..11).map(|_| rand::random::<char>()).collect();

        let query_string = format!(
            "code_verifier=deadbeef&{}={}&id=test_id",
            mock_key, mock_value
        );
        let params_map_result = parse_query_string(&query_string);
        assert!(params_map_result.is_ok());
        let params_map = params_map_result.unwrap();
        assert_eq!(params_map.get(&mock_key), Some(&mock_value));
        assert_eq!(params_map.len(), 3);
    }

    #[test]
    fn test_query_string_empty() {
        let params_map = parse_query_string(&"");
        assert!(
            params_map.is_ok(),
            "params map error {}",
            params_map.err().unwrap()
        );
        assert_eq!(params_map.unwrap().len(), 0);
    }

    #[actix_web::test]
    async fn test_health_service() {
        let app = actix_web::test::init_service(App::new().service(get_health)).await;
        let req = actix_web::test::TestRequest::get()
            .uri("/health")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let resp_body = actix_web::body::to_bytes(resp.into_body()).await;
        assert!(resp_body.is_ok());
        assert_eq!(resp_body.unwrap(), actix_web::web::Bytes::from("ok!"));
    }

    #[actix_web::test]
    async fn test_oauth2_redirect_service_ok() {
        let app = actix_web::test::init_service(App::new().service(get_oauth2_redirect)).await;
        let req = actix_web::test::TestRequest::get()
            .uri("/oauth2/redirect?code=oauth2token&code_verifier=pkce_verifier")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_oauth2_redirect_service_no_code() {
        let app = actix_web::test::init_service(App::new().service(get_oauth2_redirect)).await;
        let req = actix_web::test::TestRequest::get()
            .uri("/oauth2/redirect?code_verifier=pkce_verifier")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
