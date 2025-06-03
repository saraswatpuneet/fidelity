mod leaderboard;
#[allow(unused)]
mod lru_cache;
mod rate_limiter;

fn main() {
    rate_limiter::run_rate_limiter();
    lru_cache::main_lru();
    leaderboard::main_leader();
}
