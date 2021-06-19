pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();

    for current in string.chars() {
        match current {
            '{' | '[' | '(' => stack.push(current),
            '}' | ']' | ')' => {
                let last_opened = match stack.pop() {
                    Some(l) => l,
                    None => return false,
                };

                match (last_opened, current) {
                    ('[', ']') | ('{', '}') | ('(', ')') => continue,
                    _ => return false,
                }
            }
            _ => continue,
        }
    }

    stack.len() == 0
}
