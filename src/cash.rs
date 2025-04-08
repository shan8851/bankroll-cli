use crate::RiskLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CashStake {
    NL2,
    NL5,
    NL10,
    NL25,
    NL50,
    NL100,
    NL200,
    NL500,
    NL1000,
    NL2000,
    NL5000,
    NL10000,
}

impl CashStake {
    pub fn buyin_amount(&self) -> f64 {
        match self {
            CashStake::NL2 => 2.0,
            CashStake::NL5 => 5.0,
            CashStake::NL10 => 10.0,
            CashStake::NL25 => 25.0,
            CashStake::NL50 => 50.0,
            CashStake::NL100 => 100.0,
            CashStake::NL200 => 200.0,
            CashStake::NL500 => 500.0,
            CashStake::NL1000 => 1000.0,
            CashStake::NL2000 => 2000.0,
            CashStake::NL5000 => 5000.0,
            CashStake::NL10000 => 10000.0,
        }
    }
}

const STAKES: [CashStake; 12] = [
    CashStake::NL10000,
    CashStake::NL5000,
    CashStake::NL2000,
    CashStake::NL1000,
    CashStake::NL500,
    CashStake::NL200,
    CashStake::NL100,
    CashStake::NL50,
    CashStake::NL25,
    CashStake::NL10,
    CashStake::NL5,
    CashStake::NL2,
];

pub fn min_buyins(risk: RiskLevel) -> f64 {
    match risk {
        RiskLevel::Conservative => 50.0,
        RiskLevel::Moderate => 40.0,
        RiskLevel::Aggressive => 30.0,
        RiskLevel::UltraAggressive => 20.0,
    }
}

pub fn get_cash_stake_with_neighbors(
    bankroll: f64,
    risk: RiskLevel,
) -> (CashStake, Option<CashStake>, Option<CashStake>) {
    let min_buyins = min_buyins(risk);

    for (i, stake) in STAKES.iter().enumerate() {
        if bankroll >= stake.buyin_amount() * min_buyins {
            let lower_stake = if i < STAKES.len() - 1 {
                Some(STAKES[i + 1])
            } else {
                None
            };
            let upper_stake = if i > 0 { Some(STAKES[i - 1]) } else { None };
            return (*stake, lower_stake, upper_stake);
        }
    }

    (CashStake::NL2, None, None)
}
