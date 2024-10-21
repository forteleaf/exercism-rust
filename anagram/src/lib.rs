use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);

    possible_anagrams
        .iter()
        .filter(|&x| {
            let x_lower = x.to_lowercase();
            x.len() == word_lower.len() // 길이가 같고
                && x_lower != word_lower // 같은 단어가 아니고,
                && get_sorted(&x_lower) == word_sorted // 정렬했을 때, 같은 단어가 아님.
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted = word.chars().collect::<Vec<char>>();
    word_sorted.sort_unstable();
    word_sorted.into_iter().collect()
}
