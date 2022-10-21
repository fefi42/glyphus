pub struct Config {
    file_path: String
}

pub fn parse_args(args: &[String])-> Config {
    Config { file_path: args[1].clone() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {

        let args: Vec<String> = vec![String::from("target/program"), String::from("test_folder/my_picture.png")];
        let cfg = parse_args(&args);
        assert_eq!(cfg.file_path, "test_folder/my_picture.png");
    }
}