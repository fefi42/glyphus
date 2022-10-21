pub struct Config {
    file_path: String
}

pub fn parse_args(args: &[String])-> Result<Config, &'static str> {
    if args.len() < 2 {
        return Err("no input file path provided")
    }
    Ok(Config { file_path: args[1].clone() })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args: Vec<String> = vec![String::from("target/program"), String::from("test_folder/my_picture.png")];
        let cfg = parse_args(&args).unwrap();
        assert_eq!(cfg.file_path, "test_folder/my_picture.png");
    }

    #[test]
    fn no_arguments() {
        let args: Vec<String> = Vec::new();
        match parse_args(&args) {
            Ok(i) => assert!(false, "Expected function to fail due to missing required arguments"),
            Err(e) => assert_eq!(e, "no input file path provided"),
        }
    }

}