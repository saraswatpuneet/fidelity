use std::collections::{BinaryHeap, HashMap};

pub struct Leader {
    pub players: HashMap<u32, u32>,
}

impl Leader {
    pub fn new() -> Self {
        Leader {
            players: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, player_id: u32, score: u32) {
        *self.players.entry(player_id).or_default() += score
    }

    pub fn top_k(&mut self, k: usize) -> u32 {
        let mut heap = BinaryHeap::new();
        for &score in self.players.values() {
            heap.push(score);
        }
        heap.into_iter().take(k).sum()
    }

    pub fn reset(&mut self, player_id: &u32) {
        self.players.remove(player_id);
    }
}

pub fn main_leader() {
    let mut leaderboard = Leader::new();
    leaderboard.add_score(1, 50);
    leaderboard.add_score(2, 80);
    leaderboard.add_score(3, 70);
    println!("{}", leaderboard.top_k(2)); // 80 + 70 = 150
    leaderboard.reset(&2);
    println!("{}", leaderboard.top_k(2)); // 70 + 50 = 120
}
