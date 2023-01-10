#[macro_use] extern crate rocket;
use std::{process::Command, os::windows::process::CommandExt};

use rocket::response::content::RawHtml;
use rocket::{response::content, figment::Metadata};
use rocket_dyn_templates::{Template, context};
use std::{thread, fs};
use rocket::fs::NamedFile;
use rocket::response::status::{NotFound, self};
use std::path::PathBuf;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};

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

#[derive(FromForm, Debug)]
struct Task<'r> {
    r#prompt: &'r str,
    r#neg_prompt: &'r str,
    r#model: &'r str,
    r#scheduler: &'r str,
    steps: u8,
    guidance: f32,
    width: u16,
    height: u16,
    iterations: u16,
}

#[post("/command", data = "<task>")]
async fn command(task: Form<Task<'_>>, jar: &CookieJar<'_>) -> RawHtml<String> {
  
  println!("command : {:?}", task);

  if task.prompt.contains("&") {
    return RawHtml(format!("Forbidden character in prompt : {}", task.prompt).clone());
  } else if task.neg_prompt.contains("&") {
    return RawHtml(format!("Forbidden character in negative prompt : {}", task.neg_prompt).clone());
  } else if task.model.contains("&") || task.scheduler.contains("&") {
    return RawHtml(format!("Forbidden character in model or scheduler"));
  }

  if u32::from(task.width) * u32::from(task.height) > 262144 {
    return RawHtml(format!("Request failed due to image size, please reduce the image width ({}) or height({})", task.width, task.height).clone());
  }

  let args = format!("/C start cmd.exe /c \"cd C:\\stable_diff \
  && .\\virtualenv\\Scripts\\activate \
  && python txt2img_onnx.py \
  --model .\\model\\{} \
  --prompt \"{}\" \
  --neg_prompt \"{}\" \
  --guidance-scale \"{}\" \
  --steps {} \
  --width {} \
  --height {} \
  --output  C:\\Users\\Fluttyx\\Documents\\Dev\\StableDiff\\hello-rocket\\backend\\data\\output \
  --iterations {} \
  --scheduler {} \"", task.model, task.prompt, task.neg_prompt, task.guidance, task.steps, task.width, task.height, task.iterations, task.scheduler);

  Command::new("cmd")
      .raw_arg(args)
      .output()
      .expect("failed to execute process");

  let paths = fs::read_dir("./data/output").unwrap();

  if jar.get("other").is_none() {
    jar.add(Cookie::new("generated", (paths.count() - 2).to_string()));
  }

  RawHtml(format!("<img src=\"data\\output\\{:0>6}-00.png\" alt=\"Generated Image\">", /*(paths.count() - 2).to_string()*/ 8))
}

#[get("/lastimage")]
async fn lastimage(jar: &CookieJar<'_>) -> status::Accepted<String> {
  if let Some(cookie) = jar.get("generated") {
    status::Accepted(Some(String::from(cookie.value())))
  } else {
    status::Accepted(None)
  }
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
    .mount("/", routes![index, static_files, data, command, lastimage])
}