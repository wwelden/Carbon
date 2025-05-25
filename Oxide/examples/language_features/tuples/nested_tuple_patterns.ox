match ("user", ("Bob", 25)) {
    ("user", (name, age)) => name
    _ => "unknown"
}
