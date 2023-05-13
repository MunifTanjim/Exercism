pub fn raindrops(n: u32) -> String {
    let mut drops = vec![];

    if n % 3 == 0 {
        drops.push("Pling");
    }

    if n % 5 == 0 {
        drops.push("Plang");
    }

    if n % 7 == 0 {
        drops.push("Plong");
    }

    if drops.is_empty() {
        n.to_string()
    } else {
        drops.join("")
    }
}
