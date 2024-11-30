#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied() // last() 마지막점수 반환 Option<u32>로 반환
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied() // Option<u32>로 반환
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.clone();
        sorted_scores.sort_unstable_by(|a, b| b.cmp(a)); // 내림차순으로 정렬
        sorted_scores.truncate(3); // 상위 3개만 추출
        sorted_scores
    }
}
