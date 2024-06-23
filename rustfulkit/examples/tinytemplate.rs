use tinytemplate::TinyTemplate;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    name: String,
}

static TEMPLATE : &'static str = r#"

use axum::extract::State;

use crate::\{
    app_state::AppState,
    http::\{error::ServiceError, response::Response, OhMyResult},
};

pub async fn \{name}_handler(
    State(AppState \{ ref pool }): State<AppState>
) -> OhMyResult<Response<()>> \{
    // Do something here
    // ...
    OhMyResult::Ok(Response::Ok)
}

"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("hello", TEMPLATE)?;

    let context = Context {
        name: "World".to_string(),
    };

    let rendered = tt.render("hello", &context)?;
    println!("{}", rendered);

    Ok(())
}