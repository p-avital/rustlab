fn main() -> std::io::Result<()> {
    let path = std::env::args().skip(1).next().unwrap();
    let reader = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(reader);
    let value: serde_json::Value = serde_yaml::from_reader(reader).unwrap();
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
    Ok(())
}
