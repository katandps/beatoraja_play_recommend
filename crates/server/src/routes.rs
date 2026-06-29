mod authorization;
mod custom_table;
mod general;
mod logged_in;
use futures::lock::Mutex;
use mysql::MySqlPool;
use service::songs::SongsTag;
use std::sync::Arc;
use table::TableClient;
use warp::filters::cors::Builder;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub fn routes(db_pool: &MySqlPool, tables: TableClient) -> BoxedFilter<(impl Reply,)> {
    let songs_tag = Arc::new(Mutex::new(SongsTag::new()));

    general::routes(db_pool, tables.clone(), &songs_tag)
        .or(logged_in::routes(db_pool, &songs_tag))
        .or(authorization::routes(db_pool))
        .or(custom_table::routes(db_pool, tables))
        .with(cors_header())
        .with(warp::log("api_access"))
        .recover(handle_rejection)
        .boxed()
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let (status, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found")
    } else if err.find::<crate::filter::Unauthorized>().is_some() {
        (StatusCode::UNAUTHORIZED, "Unauthorized")
    } else if err.find::<warp::reject::MissingHeader>().is_some() {
        (StatusCode::UNAUTHORIZED, "Missing authentication header")
    } else if err.find::<crate::filter::DbUnavailable>().is_some() {
        (StatusCode::SERVICE_UNAVAILABLE, "Database unavailable")
    } else if err.find::<crate::filter::InvalidMultipart>().is_some() {
        (StatusCode::BAD_REQUEST, "Invalid multipart request")
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        (StatusCode::BAD_REQUEST, "Invalid request body")
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::PAYLOAD_TOO_LARGE, "Payload too large")
    } else {
        log::error!("unhandled rejection: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    };

    let body = serde_json::json!({ "error": message });
    Ok(warp::reply::with_status(warp::reply::json(&body), status))
}

fn cors_header() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec![
            "x-requested-with",
            "origin",
            "referer",
            "x-csrftoken",
            "oauth-token",
            "content-type",
            "content-length",
            "accept",
            "accept-encoding",
            "accept-language",
            "user-agent",
            crate::SESSION_KEY,
        ])
}

#[cfg(test)]
mod tests {
    use super::handle_rejection;
    use serde_json::Value;
    use warp::http::StatusCode;
    use warp::Filter;
    use warp::{hyper, reject, Rejection, Reply};

    async fn assert_error_response(rejection: Rejection, status: StatusCode, message: &str) {
        let reply = handle_rejection(rejection).await.unwrap();
        let response = reply.into_response();
        assert_eq!(response.status(), status);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], message);
    }

    #[tokio::test]
    async fn unauthorized_rejection_maps_to_401() {
        assert_error_response(
            reject::custom(crate::filter::Unauthorized),
            StatusCode::UNAUTHORIZED,
            "Unauthorized",
        )
        .await;
    }

    #[tokio::test]
    async fn missing_header_rejection_maps_to_401() {
        let route = warp::path::end()
            .and(crate::filter::with_login())
            .map(|_| warp::reply())
            .recover(handle_rejection);

        let response = warp::test::request().path("/").reply(&route).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let json: Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(json["error"], "Missing authentication header");
    }

    #[tokio::test]
    async fn not_found_rejection_maps_to_404() {
        assert_error_response(reject::not_found(), StatusCode::NOT_FOUND, "Not found").await;
    }

    #[tokio::test]
    async fn unknown_rejection_maps_to_500() {
        #[derive(Debug)]
        struct Unknown;
        impl reject::Reject for Unknown {}

        assert_error_response(
            reject::custom(Unknown),
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
        )
        .await;
    }
}
