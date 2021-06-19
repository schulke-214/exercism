pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];

    for current in string.chars() {
        match current {
            '{' | '[' | '(' => stack.push(current),
            '}' | ']' | ')' => match (stack.pop().unwrap_or_default(), current) {
                ('[', ']') | ('{', '}') | ('(', ')') => continue,
                _ => return false,
            },
            _ => continue,
        }
    }

    stack.is_empty()
}
