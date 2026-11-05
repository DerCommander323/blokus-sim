#[derive(Debug, Clone)]
pub struct Statistics {
    pub finished_games: i32,
    pub amount_wins: PerPlayerCounter<i32>,
    pub average_score: PerPlayerCounter<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct PerPlayerCounter<T> {
    pub blue: T,
    pub yellow: T,
    pub red: T,
    pub green: T,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            finished_games: 0,
            amount_wins: PerPlayerCounter::default(),
            average_score: PerPlayerCounter::default(),
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }
}
