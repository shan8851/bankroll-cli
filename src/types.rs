#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    Cash,
    Tournament,
    SitAndGo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Conservative,
    Moderate,
    Aggressive,
    UltraAggressive,
}

#[derive(Debug)]
pub struct PlayerProfile {
    pub bankroll: f64,
    pub game_type: GameType,
    pub risk_level: RiskLevel,
}
