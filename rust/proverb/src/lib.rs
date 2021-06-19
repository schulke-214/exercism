pub fn build_proverb(list: &[&str]) -> String {
    let ending = {
        if list.is_empty() {
            vec![]
        } else {
            vec![format!("And all for the want of a {}.", list[0])]
        }
    };

    list.iter()
        .zip(list.iter().skip(1))
        .map(|(verb, next)| format!("For want of a {} the {} was lost.\n", verb, next))
        .chain(ending)
        .collect()
}
