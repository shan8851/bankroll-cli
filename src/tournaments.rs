use crate::{GameType, RiskLevel};

pub fn get_tournament_stake(bankroll: f64, game: GameType, risk: RiskLevel) -> f64 {
    let min_buyins = match (game, risk) {
        (GameType::SitAndGo, RiskLevel::Conservative) => 100.0,
        (GameType::SitAndGo, RiskLevel::Moderate) => 50.0,
        (GameType::SitAndGo, RiskLevel::Aggressive) => 30.0,
        (GameType::SitAndGo, RiskLevel::UltraAggressive) => 20.0,
        (GameType::Tournament, RiskLevel::Conservative) => 200.0,
        (GameType::Tournament, RiskLevel::Moderate) => 100.0,
        (GameType::Tournament, RiskLevel::Aggressive) => 50.0,
        (GameType::Tournament, RiskLevel::UltraAggressive) => 30.0,
        (GameType::Cash, _) => {
            panic!("get_tournament_stake() called with GameType::Cash");
        }
    };

    bankroll / min_buyins
}
