use std::io;
mod cash;
use cash::*;
mod tournaments;
use tournaments::get_tournament_stake;


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


    match player_profile.game_type {
    GameType::Cash => {
        let (current, move_down, move_up) =
            get_cash_stake_with_neighbors(player_profile.bankroll, player_profile.risk_level);

        println!("🎯 Recommended Stake: {:?}", current);

        if let Some(up) = move_up {
            println!(
                "🟢 Move up to {:?} at: ${:.2}",
                up,
                up.buyin_amount() * min_buyins(player_profile.risk_level)
            );
        }

        if let Some(down) = move_down {
            println!(
                "🔻 Move down to {:?} if below: ${:.2}",
                down,
                down.buyin_amount() * min_buyins(player_profile.risk_level)
            );
        }
    }

    GameType::SitAndGo | GameType::Tournament => {
        let stake = get_tournament_stake(
            player_profile.bankroll,
            player_profile.game_type,
            player_profile.risk_level,
        );

        match player_profile.game_type {
            GameType::SitAndGo => {
                println!("🎯 You should target SNGs with a buy-in around: ${:.2}", stake);
            }
            GameType::Tournament => {
                println!(
                    "🎯 Your average tournament buy-in should be around: ${:.2}",
                    stake
                );
            }
            _ => {}
        }
    }
}

}
