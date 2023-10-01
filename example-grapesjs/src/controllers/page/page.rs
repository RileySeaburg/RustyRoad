use actix_identity::Identity;
use actix_web::{post, patch, web, HttpResponse, get};
use crate::models::Page;
/// Alert: This is a generated controller.
/// The controller is generated by the rustyroad CLI.
/// It is a best guess at what the controller should look like.
/// Please review the controller and make any necessary changes.
#[post("/page")]
pub async fn create_page(page: web::Json<Page>,user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let result = Page::create_page(page.into_inner()).await;
        match result {
            Ok(page) => HttpResponse::Ok().json(page),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    } else {
      // redirect to login page
      let mut context = tera::Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to create a new page.");
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish()
    }
   // before we allow the user to create a new page we need to check if they are logged in
   // if they are not logged in, we need to redirect them to the login page
}

#[get("/page")]
pub async fn get_create_page_view(tmpl: web::Data<tera::Tera>, user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let mut context = tera::Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("title", "create_page");
        context.insert("controller_name", "create_page");
        let rendered = tmpl.render("layouts/authenticated/page/create_page.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        let mut context = tera::Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish()
    }
}



/// Alert: This is a generated controller.
/// The controller is generated by the rustyroad CLI.
/// It is a best guess at what the controller should look like.
/// Please review the controller and make any necessary changes.
#[patch("/page")]
pub async fn update_page(id: web::Path<i32>, page: web::Json<Page>, user: Option<Identity>) -> HttpResponse {
    if let Some(_user) = user {
        let result = Page::update_page(*id, page.into_inner()).await;
        match result {
            Ok(page) => HttpResponse::Ok().json(page),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    } else {
        let mut context = tera::Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to update a page.");
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish()
    }
}