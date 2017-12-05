use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

pub fn start(port: u32) {
    println!("Starting thumbnailator service on port {}", port);

    let mut router = Router::new();
    router.post("/generate", generate_thumbnails, "generate");

    Iron::new(router)
        .http(format!("localhost:{}", port))
        .unwrap();
}

fn generate_thumbnails(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    println!("Received payload: {}", &payload);

    let json: GenerateRequest = ::serde_json::from_str(&payload).unwrap();
    println!("JSON: {:?}", &json);

    Ok(Response::with(status::Ok))
}

#[derive(Serialize, Deserialize, Debug)]
struct GenerateRequest {
    images: Vec<String>,
    size: u32,
}