pub mod file_read {
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Result};
    use std::path::Path;

    pub fn read_file(path: &String) -> Vec<Result<String>> {
        let path = Path::new(path);
        let file = match File::open(&path) {
            Err(err) =>
                panic!("Could not open file {}: {}", path.display(), <dyn Error>::to_string(&err)),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();

        return lines;
    }

    pub fn file_path(exercise_name: &String) -> String {
        return format!("./src/{}/data.txt", exercise_name);
    }
}
