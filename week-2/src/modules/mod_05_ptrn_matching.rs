// parsing patterns
fn parse_command(input: &str) -> String {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Write fn parse_command(input: &str) -> String:

    // "quit" → "Goodbye"
    // "echo ..." → everything after "echo" joined with spaces
    // "add X Y" → sum of X and Y as string
    // "repeat N MSG" → MSG repeated N times, space-separated
    // anything else → "Unknown"
    // Match on tokens.as_slice()
    match tokens.as_slice() {
        ["quit"] => "Goodbye".to_string(),
        ["echo", rest @ ..] => rest.join(" "),
        ["add", a, b] => match (a.parse::<i32>(), b.parse::<i32>()) {
            (Ok(a), Ok(b)) => (a + b).to_string(),
            _ => "Invalid Number".to_string(),
        },
        ["repeat", x, rest @ ..] => {
            let count = x.parse::<usize>().unwrap_or(0);
            let msg = rest.join(" ");

            std::iter::repeat(msg)
                .take(count)
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => "Unkown".to_string(),
    }
}

// classifying
fn classify(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        val @ 1..=10 => format!("small: {}", val),
        val @ -9..=-1 => format!("neg small: {}", val),
        _ => format!("big: {}", n),
    }
}
