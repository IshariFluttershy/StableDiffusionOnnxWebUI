#[macro_use] extern crate rocket;
use std::{process::Command, os::windows::process::CommandExt};

use rocket::{response::content, figment::Metadata};
use rocket_dyn_templates::{Template, context};
use std::thread;

#[get("/")]
fn index() -> &'static str {
  "Hello world"
}

#[get("/command")]
fn command() -> &'static str {
  let output = Command::new("cmd")
      .raw_arg("/C start cmd.exe /c \"cd C:\\stable_diff && .\\virtualenv\\Scripts\\activate && python txt2img_onnx.py --model .\\model\\waifu-diffusion-diffusers-onnx-v1-3 --prompt \"1girl anime smile blue_hair\" --steps 10 --scheduler eulera \"")
      .output()
      .expect("failed to execute process");

      println!("status: {}", output.status);
      println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
  "La commande"
}

#[launch]
fn rocket() -> _ {

  /*thread::spawn(move || {
    let output = Command::new("cmd")
      .raw_arg("/C start cmd.exe /k \"cd C:\\stable_diff && .\\virtualenv\\Scripts\\activate && python onnxUI.py\"")
      .output()
      .expect("failed to execute process");

      println!("status: {}", output.status);
      println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
  });*/

    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![command])
}