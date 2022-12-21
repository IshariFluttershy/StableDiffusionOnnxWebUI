#[macro_use] extern crate rocket;
use std::{process::Command, os::windows::process::CommandExt};

use rocket::{response::content, figment::Metadata};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> &'static str {
  "Hello world"
}

#[get("/command")]
fn command() -> &'static str {
  "La commande"
  /*if let Ok(o) = Command::new("cmd")
    .args(["/C", "timeout 5"])
    .output() {
      println!("{:?}", o);
      "Ca fait une commande !!!"
    } else {
      "Eh non"
    }*/
}

#[launch]
fn rocket() -> _ {

  //&& python onnxUI.py
  /*let output = Command::new("cmd")
  //.args(["/C", "cd C:\\stable_diff && .\\virtualenv\\Scripts\\activate "])
  .args(["/C", "start cmd.exe"])
  .output()
  .expect("failed to execute process");*/

  let output = Command::new("cmd")
    .raw_arg("/C start cmd.exe /k \"cd C:\\stable_diff && .\\virtualenv\\Scripts\\activate && python onnxUI.py\"")
    .output()
    .expect("failed to execute process");


  //let hello = output.stdout;

  println!("status: {}", output.status);
  println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
  println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![command])
}