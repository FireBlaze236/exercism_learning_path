use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
pub struct HighScores {
    all_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            all_scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.all_scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.all_scores.as_slice().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.all_scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut heap = BinaryHeap::with_capacity(4);

        for &score in self.all_scores.as_slice() {
            heap.push(Reverse(score));
            if heap.len() > 3 {
                heap.pop();
            }
        }

        let mut result: Vec<u32> = heap.into_iter().map(|Reverse(x)| x).collect();

        result.sort_unstable_by(|a, b| b.cmp(a));

        result
    }
}
