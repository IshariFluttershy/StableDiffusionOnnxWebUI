#[macro_use] extern crate rocket;
use std::{process::Command, os::windows::process::CommandExt};

use rocket::response::content::RawHtml;
use rocket::{response::content, figment::Metadata};
use rocket_dyn_templates::{Template, context};
use std::{thread, fs};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;
use rocket::form::Form;

#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./data/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
  NamedFile::open("../ui/dist/index.html")
      .await
      .map_err(|e| NotFound(e.to_string()))
}

//Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
  let path = PathBuf::from("../ui/dist").join(path);
  match NamedFile::open(path).await {
      Ok(f) => Ok(f),
      Err(_) => get_index().await,
  }
}

// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
  get_index().await
}

#[derive(FromForm)]
struct Task<'r> {
    r#prompt: &'r str,
    r#neg_prompt: &'r str,
    steps: u8,
    guidance: f32,
    width: u16,
    height: u16,
}

#[post("/command", data = "<task>")]
async fn command(task: Form<Task<'_>>) -> RawHtml<String> {
  
  println!("prompt : {} \n steps : {}", task.prompt, task.steps);

  if task.prompt.contains("&") {
    return RawHtml(format!("Forbidden character in prompt : {}", task.prompt).clone());
  } else if task.neg_prompt.contains("&") {
    return RawHtml(format!("Forbidden character in negative prompt : {}", task.neg_prompt).clone());
  }

  if u32::from(task.width) * u32::from(task.height) > 262144 {
    return RawHtml(format!("Request failed due to image size, please reduce the image width ({}) or height({})", task.width, task.height).clone());
  }

  let args = format!("/C start cmd.exe /c \"cd C:\\stable_diff \
  && .\\virtualenv\\Scripts\\activate \
  && python txt2img_onnx.py \
  --model .\\model\\waifu-diffusion-diffusers-onnx-v1-3 \
  --prompt \"{}\" \
  --neg_prompt \"{}\" \
  --guidance-scale \"{}\" \
  --steps {} \
  --width {} \
  --height {} \
  --output  C:\\Users\\Fluttyx\\Documents\\Dev\\StableDiff\\hello-rocket\\backend\\data\\output \
  --scheduler eulera \"", task.prompt, task.neg_prompt, task.guidance, task.steps, task.width, task.height);

  //let task = thread::spawn(move || {
    let output = Command::new("cmd")
        .raw_arg(args)
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
  //});

  let paths = fs::read_dir("./data/output").unwrap();

  RawHtml(format!("<img src=\"data\\output\\{:0>6}-00.png\" alt=\"Generated Image\">", (paths.count() - 2).to_string()))
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
    .mount("/", routes![index, static_files, data, command])
}