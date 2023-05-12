fn capitalize(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn bottle_count(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        n => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    let before_count = bottle_count(n);

    let instruction = if n == 0 {
        "Go to the store and buy some more"
    } else if n == 1 {
        "Take it down and pass it around"
    } else {
        "Take one down and pass it around"
    };

    String::from(format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
        capitalize(&before_count),
        before_count,
        instruction,
        bottle_count((n + 99) % 100)
    ))
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
