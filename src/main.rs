#[macro_use] extern crate rocket;
use rocket::{response::content, figment::Metadata};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    //Metadata::render("home", context! { first_name: "Tanguy", last_name: "Morvant" }))
    content::RawHtml(
        r#"<head>
        <style type="text/css">
          body {
            background: #1a6875;
            font-family: Arial, Helvetica, sans-serif;
          }
          .left-side {
            max-width: 45%;
            text-align: right;
            position: absolute;
            top: 15%;
            left: 0;
          }
          .title {
            color: #fff;
            font-size: 5em;
          }
          .sub-title {
            color: #fff;
            font-size: 1.3em;
          }
          .right-side {
            width: 30em;
            height: 30em;
            background: #d0e64c;
            border-radius: 50%;
            position: absolute;
            top: 10%;
            right: 5%;
          }
        </style>
      </head>
      <body>
        <div class="left-side">
          <h1 class="title">This web app was built with RUST</h1>
          <p class="sub-title">Hello {{first_name}} {{last_name}}😊</p>
        </div>
        <div class="right-side">
          <div class="round-image"></div>
        </div>
      </body>"#
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .attach(Template::fairing())
}