use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);

    possible_anagrams
        .iter()
        .filter(|&x| {
            let x_lower = x.to_lowercase();
            x.len() == word_lower.len()
                && x_lower != word_lower
                && get_sorted(&x_lower) == word_sorted
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted = word.chars().collect::<Vec<char>>();
    word_sorted.sort_unstable();
    word_sorted.into_iter().collect()
}
