pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|items| format!("For want of a {} the {} was lost.", items[0], items[1]))
        .chain(std::iter::once(if let Some(item) = list.first() {
            format!("And all for the want of a {}.", item)
        } else {
            String::new()
        }))
        .collect::<Vec<String>>()
        .join("\n")
}
