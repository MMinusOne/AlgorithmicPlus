use std::error::Error;

use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};

pub struct ConsecutiveWinsLosses {
    name: String,
    description: String,
    consecutive_wins: i32,
    consecutive_losses: i32,
    most_consecutive_wins: i32,
    most_consecutive_losses: i32,
}

impl IInjectable<f32, (i32, i32)> for ConsecutiveWinsLosses {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, pl_ratio: f32) {
        if pl_ratio > 0.0 {
            self.consecutive_wins += 1;
            self.consecutive_losses = 0;
            if self.consecutive_wins > self.most_consecutive_wins {
                self.most_consecutive_wins = self.consecutive_wins;
            }
        } else if pl_ratio < 0.0 {
            self.consecutive_losses += 1;
            self.consecutive_wins = 0;

            if self.consecutive_losses > self.most_consecutive_losses {
                self.most_consecutive_losses = self.consecutive_losses;
            }
        }
    }

    fn get_data(&mut self) -> Option<(i32, i32)> {
        Some((self.most_consecutive_wins, self.most_consecutive_losses))
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl ConsecutiveWinsLosses {
    pub fn new() -> Self {
        return Self {
            name: "ConsecutiveWinsLosses".into(),
            description: "Consecutive wins and losses".into(),
            consecutive_wins: 0,
            consecutive_losses: 0,
            most_consecutive_wins: 0,
            most_consecutive_losses: 0,
        };
    }
}
