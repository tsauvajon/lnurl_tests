use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::invoice::Invoice;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/invoice", post(create_invoice))
}

// TODO statically serve the frontend
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_invoice(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateInvoice` type
    Json(payload): Json<CreateInvoice>,
) -> Result<Json<CreatedInvoice>, AppError> {
    Invoice::with_amount("timebrand03@walletofsatoshi.com", payload.sats)
        .await
        .map(|invoice| Json(invoice.into()))
        .map_err(Into::into)
}

// the input to our `create_invoice` handler
#[derive(Deserialize)]
struct CreateInvoice {
    /// Amount, in sats
    sats: u64,
}

// the output to our `create_invoice` handler
#[derive(Serialize)]
struct CreatedInvoice {
    data: String,
}

impl From<Invoice> for CreatedInvoice {
    fn from(invoice: Invoice) -> Self {
        CreatedInvoice {
            data: invoice.data(),
        }
    }
}

struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(r#"{{ "msg": "{}" }}"#, self.0),
        )
            .into_response()
    }
}

impl From<crate::invoice::Error> for AppError {
    fn from(err: crate::invoice::Error) -> Self {
        AppError(format!(r#"{{ "msg": "{err:?}" }}"#))
    }
}
