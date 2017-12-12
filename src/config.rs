pub struct Config {
    pub images: Vec<String>,
    pub size: u32,
    pub prefix: String,
    pub start_service: bool,
    pub service_port: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {

        let mut iter = args.iter();
        let mut cfg = Config {
            images: Vec::new(),
            size: 200,
            prefix: String::from("thumb_"),
            start_service: false,
            service_port: 8080,
        };

        loop {
            match iter.next() {
                Some(arg) => {
                    let arg: Vec<&str> = arg.split('=').collect();
                    if arg.len() != 2 && arg[0] != "--start-service" {
                        return Err(format!("Invalid argument: {:?}", arg));
                    }
                    match arg[0] {
                        "--size" => cfg.size = parse_u32(arg[1])?,
                        "--images" => cfg.images = arg[1].split(',').map(String::from).collect(),
                        "--prefix" => cfg.prefix = String::from(arg[1]),
                        "--start-service" => cfg.start_service = true,
                        "--service-port" => cfg.service_port = parse_u32(arg[1])?,
                        _ => return Err(format!("Unrecognized argument: {}", arg[0])),
                    }
                }
                None => break,
            }
        }

        if !cfg.start_service && cfg.images.is_empty() {
            return Err(String::from("No images provided."));
        }

        Ok(cfg)
    }
}

fn parse_u32(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(size) => return Ok(size),
        Err(_) => return Err(format!("Argument '{}' is not a valid number.", s)),
    };
}

#[cfg(test)]
mod tests {

    use ::*;

    #[test]
    fn should_return_error_when_argument_witohut_value() {
        let args: Vec<String> = vec![String::from("--size")];

        let cfg = Config::new(&args);

        assert!(cfg.is_err());
    }

    #[test]
    fn should_create_config_with_correct_arguments() {
        let args: Vec<String> = vec![
            String::from("--size=100"),
            String::from("--images=/tmp/foo.jpg,/tmp/bar.png"),
            String::from("--prefix=gnome_"),
            String::from("--start-service"),
            String::from("--service-port=9966"),
        ];

        let cfg = Config::new(&args).unwrap();

        assert_eq!(cfg.size, 100);
        assert_eq!(cfg.images, vec!["/tmp/foo.jpg", "/tmp/bar.png"]);
        assert_eq!(cfg.prefix, "gnome_");
        assert_eq!(cfg.start_service, true);
        assert_eq!(cfg.service_port, 9966);
    }

    #[test]
    fn should_pass_if_defult_arguments_not_provided() {
        let args: Vec<String> = vec![String::from("--images=/tmp/foo.jpg,/tmp/bar.png")];

        let cfg = Config::new(&args).unwrap();

        assert_eq!(cfg.size, 200);
        assert_eq!(cfg.images, vec!["/tmp/foo.jpg", "/tmp/bar.png"]);
        assert_eq!(cfg.prefix, "thumb_");
        assert_eq!(cfg.start_service, false);
        assert_eq!(cfg.service_port, 8080);
    }

    #[test]
    fn should_failed_parse_config_if_no_images_provided() {
        let args: Vec<String> = Vec::new();

        let cfg = Config::new(&args);

        assert!(cfg.is_err());
    }

    #[test]
    fn should_parse_config_if_no_images_provided_and_service_is_going_to_be_started() {
        let args: Vec<String> = vec![
            String::from("--start-service"),
            String::from("--service-port=9966"),
        ];

        let cfg = Config::new(&args);

        assert!(cfg.is_ok());
    }
}
