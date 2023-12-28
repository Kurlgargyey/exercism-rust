pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let index = match _student {
        "Alice" => Some(0),
        "Bob" => Some(2),
        "Charlie" => Some(4),
        "David" => Some(6),
        "Eve" => Some(8),
        "Fred" => Some(10),
        "Ginny" => Some(12),
        "Harriet" => Some(14),
        "Ileana" => Some(16),
        "Joseph" => Some(18),
        "Kincaid" => Some(20),
        "Larry" => Some(22),
        _ => None,
    }
    .unwrap();
    //panics if given a name not on student roster!
    _diagram
        .split("\n")
        .map(|row| {
            row.get(index..index + 2).unwrap()
            //panics if the garden diagram is not filled until and including student's paddies!
        })
        .collect::<String>()
        .chars()
        .map(|abbr| {
            match abbr {
                'V' => Some("violets"),
                'G' => Some("grass"),
                'R' => Some("radishes"),
                'C' => Some("clover"),
                _ => Some("unknown"),
            }
            .unwrap()
        })
        .collect()
}
