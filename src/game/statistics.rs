#[derive(Debug, Clone)]
pub struct Statistics {
    pub finished_games: i32,
    pub amount_wins: PerPlayerCounter<i32>,
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
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);

        println!("Win rates:");
        println!(
            " -   Blue: {:.2}%",
            self.amount_wins.blue as f64 / self.finished_games as f64 * 100.
        );
        println!(
            " - Yellow: {:.2}%",
            self.amount_wins.yellow as f64 / self.finished_games as f64 * 100.
        );
        println!(
            " -    Red: {:.2}%",
            self.amount_wins.red as f64 / self.finished_games as f64 * 100.
        );
        println!(
            " -  Green: {:.2}%",
            self.amount_wins.green as f64 / self.finished_games as f64 * 100.
        );
    }
}
