//side effect
// changes to external state that are caused by the functions


fn write_to_file(data:&str, filename:&str) {
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write to file");
}