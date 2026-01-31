pub fn read_csv(file_path: &str) -> Vec<Vec<String>> {
    let mut reader = csv::Reader::from_path(file_path).unwrap();

    reader
        .records()
        .map(|r| r.unwrap().iter().map(|s| s.to_string()).collect())
        .collect()
}
