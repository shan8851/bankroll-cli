use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameType {
    Cash,
    Tournament,
    SitAndGo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RiskLevel {
    Conservative,
    Moderate,
    Aggressive,
    UltraAggressive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CashStake {
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
    fn buyin_amount(&self) -> f64 {
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

fn get_cash_stake(bankroll: f64, risk: RiskLevel) -> CashStake {
    let min_buyins = match risk {
        RiskLevel::Conservative => 50.0,
        RiskLevel::Moderate => 40.0,
        RiskLevel::Aggressive => 30.0,
        RiskLevel::UltraAggressive => 20.0,
    };

    let stakes = [
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

    for stake in stakes {
        if bankroll >= stake.buyin_amount() * min_buyins {
            return stake;
        }
    }

    CashStake::NL2 // fallback
}

fn get_cash_stake_with_neighbors(
    bankroll: f64,
    risk: RiskLevel,
) -> (CashStake, Option<CashStake>, Option<CashStake>) {
    let min_buyins = match risk {
        RiskLevel::Conservative => 50.0,
        RiskLevel::Moderate => 40.0,
        RiskLevel::Aggressive => 30.0,
        RiskLevel::UltraAggressive => 20.0,
    };

    let stakes = [
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

    for (i, stake) in stakes.iter().enumerate() {
        if bankroll >= stake.buyin_amount() * min_buyins {
            let lower_stake = if i < stakes.len() - 1 {
                Some(stakes[i + 1])
            } else {
                None
            };
            let upper_stake = if i > 0 { Some(stakes[i - 1]) } else { None };
            return (*stake, lower_stake, upper_stake);
        }
    }

    (CashStake::NL2, None, None)
}

fn main() {
    #[derive(Debug)]
    struct PlayerProfile {
        bankroll: f64,
        game_type: GameType,
        risk_level: RiskLevel,
    }

    println!("What is your current bankroll.");

    let mut bank = String::new();

    io::stdin()
        .read_line(&mut bank)
        .expect("Failed to read line");

    let bankroll_amount: f64 = match bank.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter only a number.");
            return;
        }
    };

    println!("You entered: {}", bankroll_amount);

    println!(
        "What type of game do you play?\n\
1. Cash\n\
2. Tournament\n\
3. SitAndGo"
    );

    let mut game_type_input = String::new();

    io::stdin()
        .read_line(&mut game_type_input)
        .expect("Failed to read line");

    let game_type: GameType = match game_type_input.trim() {
        "1" => GameType::Cash,
        "2" => GameType::Tournament,
        "3" => GameType::SitAndGo,
        _ => {
            println!("Invalid input. Please enter 1, 2, or 3.");
            return;
        }
    };
    println!("You entered: {:?}", game_type);

    println!(
        "What is your risk level?\n\
1. Conservative\n\
2. Moderate\n\
3. Aggressive\n\
4. UltraAggressive"
    );

    let mut risk_level_input = String::new();

    io::stdin()
        .read_line(&mut risk_level_input)
        .expect("Failed to read line");

    let risk_level: RiskLevel = match risk_level_input.trim() {
        "1" => RiskLevel::Conservative,
        "2" => RiskLevel::Moderate,
        "3" => RiskLevel::Aggressive,
        "4" => RiskLevel::UltraAggressive,
        _ => {
            println!("Invalid input. Please enter 1, 2, 3, or 4.");
            return;
        }
    };
    println!("You entered: {:?}", risk_level);

    let player_profile = PlayerProfile {
        bankroll: bankroll_amount,
        game_type,
        risk_level,
    };

    println!("Player Profile: {:?}", player_profile);

    if let GameType::Cash = player_profile.game_type {
        let (current, move_down, move_up) =
            get_cash_stake_with_neighbors(player_profile.bankroll, player_profile.risk_level);

        println!("🎯 Recommended Stake: {:?}", current);

        if let Some(up) = move_up {
            println!(
                "🟢 Move up to {:?} at: ${:.2}",
                up,
                up.buyin_amount()
                    * match player_profile.risk_level {
                        RiskLevel::Conservative => 50.0,
                        RiskLevel::Moderate => 40.0,
                        RiskLevel::Aggressive => 30.0,
                        RiskLevel::UltraAggressive => 20.0,
                    }
            );
        }

        if let Some(down) = move_down {
            println!(
                "🔻 Move down to {:?} if below: ${:.2}",
                down,
                down.buyin_amount()
                    * match player_profile.risk_level {
                        RiskLevel::Conservative => 50.0,
                        RiskLevel::Moderate => 40.0,
                        RiskLevel::Aggressive => 30.0,
                        RiskLevel::UltraAggressive => 20.0,
                    }
            );
        }
    }
}
