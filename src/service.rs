use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use std::error::Error;

use thumb;

pub fn start(port: u32) {
    println!("Starting thumbnailator service on port {}", port);

    let mut router = Router::new();
    router.post("/generate", handle_generate_request, "generate");

    Iron::new(router)
        .http(format!("localhost:{}", port))
        .unwrap();
}

fn handle_generate_request(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    match request.body.read_to_string(&mut payload) {
        Ok(_) => {
            let json = match ::serde_json::from_str::<GenerateRequest>(&payload) {
                Ok(json) => json,
                Err(e) => return Err(bad_request(e, "Failed to deserialize request.")),
            };
            match generate_thumbs(json) {
                Ok(_) => Ok(Response::with(status::Ok)),
                Err(e) => Ok(unprocessable_entity(e)),
            }
        }
        Err(e) => Err(bad_request(e, "Failed to read request.")),
    }
}

fn generate_thumbs(json: GenerateRequest) -> Result<(), Vec<String>> {
    thumb::generate(
        json.images,
        json.prefix.unwrap_or(String::from("thumb_")),
        json.size.unwrap_or(200),
    )
}

fn unprocessable_entity(errors: Vec<String>) -> Response {
    Response::with((status::UnprocessableEntity, format!("{:?}", errors)))
}

fn bad_request<E: 'static + Error + Send>(error: E, message: &'static str) -> IronError {
    IronError {
        error: Box::new(error),
        response: Response::with((status::BadRequest, message)),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GenerateRequest {
    images: Vec<String>,
    #[serde(default)]
    size: Option<u32>,
    #[serde(default)]
    prefix: Option<String>,
}
