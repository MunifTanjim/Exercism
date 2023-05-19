pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_terminator(|ch| " -_".contains(ch))
        .map(|str| {
            str.chars()
                .take(1)
                .map(|c| c.to_ascii_uppercase())
                .chain(
                    str.chars()
                        .skip(1)
                        .skip_while(|c| c.is_ascii_uppercase())
                        .filter_map(|c| c.is_ascii_uppercase().then_some(c.to_ascii_uppercase())),
                )
                .collect::<String>()
        })
        .collect()
}
