pub struct Config {
    pub query: String,
    pub in_file_path: String,
    pub out_file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let in_file_path = args[2].clone();
        let out_file_path = format!("{in_file_path}_out.txt");

        Ok(Config { query, in_file_path, out_file_path})
    }
}
