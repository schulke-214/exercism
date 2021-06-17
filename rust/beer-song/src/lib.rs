#![feature(iter_intersperse)]

pub fn verse(n: u32) -> String {
    match n {
        2..=u32::MAX => format!(
            "{current} bottles of beer on the wall, {current} bottles of beer.\nTake one down and pass it around, {next} {next_bottle_pluralized} of beer on the wall.\n",
            current = n,
            next = n - 1,
            next_bottle_pluralized = { if n - 1 > 1 { "bottles" } else { "bottle" } }
        ),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        0 => String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .intersperse("\n".to_owned())
        .collect()
}
