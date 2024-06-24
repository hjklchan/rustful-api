use serde::Serialize;

use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct HandlerContext<'a> {
    operation: &'a str,
}

// Define a TEMPLATE for creating handler
static HANDLER_TEMPLATE : &'static str = r#"
use axum::extract::State;

use crate::\{
    app_state::AppState,
    http::\{error::ServiceError, response::Response, OhMyResult},
};

pub async fn {operation}_handler(
    State(AppState \{ ref pool }): State<AppState>
) -> OhMyResult<Response<()>> \{
    // Do something here
    // ...
    OhMyResult::Ok(Response::Ok)
}

"#;

pub fn make(operation: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("handler", HANDLER_TEMPLATE)?;

    let context = HandlerContext {
        operation: operation.as_ref(),
    };

    let rendered = tt.render("handler", &context)?;

    Ok(rendered)
}