use serde::Serialize;

use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct HandlerContext<'a> {
    operation: &'a str,
}

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

pub fn make_handler_content(operation: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("handler", HANDLER_TEMPLATE)?;

    let context = HandlerContext {
        operation: operation.as_ref(),
    };

    let rendered = tt.render("handler", &context)?;

    Ok(rendered)
}

#[derive(Serialize)]
pub struct ModRsContext<'a> {
    module_name: &'a str,
}

static MOD_RS_TEMPLATE : &'static str = r#"
use crate::app_state::AppState;
use axum::\{routing, Router};

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn routes(app_state: AppState) -> Router \{
    Router::new()
        .route("/{module_name}", routing::post(create::create_handler))
        .route("/{module_name}/:{module_name}_id", routing::delete(delete::delete_handler))
        .route("/{module_name}/:{module_name}_id", routing::put(update::update_handler))
        .route("/{module_name}/:{module_name}_id", routing::get(get::get_handler))
        .route("/{module_name}", routing::get(list::list_handler))
        .with_state(app_state)
}

"#;

pub fn make_mod_rs_content(module_name: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("mod_rs", MOD_RS_TEMPLATE)?;

    let context = ModRsContext {
        module_name: module_name.as_ref(),
    };

    let rendered = tt.render("mod_rs", &context)?;

    println!("{rendered}");

    Ok(rendered)
}
