use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let word_hash = word.chars().collect::<Vec<_>>();
    let mut result: HashSet<&str> = HashSet::new();

    // 각 단어가의-character-리스트를 해시 테이블에 넣어두고,
    for anagram in possible_anagrams {
        let anagram_hash = anagram.chars().collect::<Vec<_>>();

        if anagram_hash == word_hash {
            result.insert(anagram);
        }
    }

    result
}
