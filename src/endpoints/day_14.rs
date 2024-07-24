use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

use crate::common::{EndpointRet, ServerError};

#[derive(Serialize, Deserialize)]
struct TemplateContext {
    content: String,
}

static TEMPLATE: &'static str = "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {content}
  </body>
</html>";

#[post("14/unsafe")]
async fn render_unsafe(body: web::Json<TemplateContext>) -> EndpointRet {
    let context = body.into_inner();
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);

    match tt.add_template("unsafe", TEMPLATE) {
        Err(_) => return Err(ServerError::InternalError),
        _ => (),
    }

    let rendered = match tt.render("unsafe", &context) {
        Ok(body) => body,
        Err(_) => return Err(ServerError::InternalError),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

#[post("14/safe")]
async fn render_safe(body: web::Json<TemplateContext>) -> EndpointRet {
    let context = body.into_inner();
    let mut tt = TinyTemplate::new();

    match tt.add_template("safe", TEMPLATE) {
        Err(_) => return Err(ServerError::InternalError),
        _ => (),
    }

    let rendered = match tt.render("safe", &context) {
        Ok(body) => body,
        Err(_) => return Err(ServerError::InternalError),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}