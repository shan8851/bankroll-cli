use std::io;

mod cash;
use cash::*;

mod tournaments;
use tournaments::get_tournament_stake;

mod types;
use crate::types::*;

use colored::*;
use figlet_rs::FIGfont;
use inquire::Select;
use terminal_size::{Width, terminal_size};

// Styled banner
fn print_banner() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("💰 Bankroll CLI 💸").unwrap().to_string();
    let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80);

    for (i, line) in figure.lines().enumerate() {
        let padding = (width.saturating_sub(line.len() as u16) / 2) as usize;
        let styled = match i % 2 {
            0 => line.truecolor(255, 215, 0).bold(), // gold
            _ => line.truecolor(0, 255, 150).bold(), // emerald
        };
        println!("{}{}", " ".repeat(padding), styled);
    }

    println!("{}", "═".repeat(width as usize).dimmed());
}

// Risk level → colored label
fn format_risk_level(risk: RiskLevel) -> ColoredString {
    match risk {
        RiskLevel::Conservative => "Conservative".green().bold(),
        RiskLevel::Moderate => "Moderate".yellow().bold(),
        RiskLevel::Aggressive => "Aggressive".red().bold(),
        RiskLevel::UltraAggressive => "UltraAggressive".bright_magenta().bold(),
    }
}

// Game type → colored label
fn format_game_type(game: GameType) -> ColoredString {
    match game {
        GameType::Cash => "Cash Games".cyan().bold(),
        GameType::SitAndGo => "Sit & Go".bright_blue().bold(),
        GameType::Tournament => "MTT".bright_magenta().bold(),
    }
}

fn main() {
    print_banner();

    println!("{}", "💵 Enter your current bankroll:".bold());

    let mut bank = String::new();
    io::stdin()
        .read_line(&mut bank)
        .expect("Failed to read line");

    let bankroll_amount: f64 = match bank.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "❌ Please enter a valid number.".red().bold());
            return;
        }
    };

    println!(
        "{} {}\n",
        "✅ Bankroll:".bold(),
        format!("${:.2}", bankroll_amount).truecolor(255, 215, 0)
    );

    // Game type prompt
    let game_str = Select::new(
        "🎮 What type of game do you play?",
        vec!["Cash", "Tournament", "SitAndGo"],
    )
    .prompt();

    let game_type = match game_str {
        Ok("Cash") => GameType::Cash,
        Ok("Tournament") => GameType::Tournament,
        Ok("SitAndGo") => GameType::SitAndGo,
        _ => {
            println!("{}", "❌ Input cancelled.".red());
            return;
        }
    };

    println!("🎯 Game Type: {}\n", format_game_type(game_type));

    // Risk level prompt
    let risk_str = Select::new(
        "🚦 What is your risk level?",
        vec!["Conservative", "Moderate", "Aggressive", "UltraAggressive"],
    )
    .prompt();

    let risk_level = match risk_str {
        Ok("Conservative") => RiskLevel::Conservative,
        Ok("Moderate") => RiskLevel::Moderate,
        Ok("Aggressive") => RiskLevel::Aggressive,
        Ok("UltraAggressive") => RiskLevel::UltraAggressive,
        _ => {
            println!("{}", "❌ Input cancelled.".red());
            return;
        }
    };

    println!("📈 Risk Level: {}\n", format_risk_level(risk_level));

    let player_profile = PlayerProfile {
        bankroll: bankroll_amount,
        game_type,
        risk_level,
    };

    println!("{}", "🧠 Calculating your optimal stake...".italic());

    match player_profile.game_type {
        GameType::Cash => {
            let (current, move_down, move_up) =
                get_cash_stake_with_neighbors(player_profile.bankroll, player_profile.risk_level);

            println!(
                "\n🎯 Recommended Stake: {}",
                format!("{:?}", current).truecolor(0, 255, 120).bold()
            );

            if let Some(up) = move_up {
                println!(
                    "🟢 Move up to {} at: {}",
                    format!("{:?}", up).bold(),
                    format!(
                        "${:.2}",
                        up.buyin_amount() * min_buyins(player_profile.risk_level)
                    )
                    .bright_green()
                );
            }

            if let Some(down) = move_down {
                println!(
                    "🔻 Move down to {} if below: {}",
                    format!("{:?}", down).bold(),
                    format!(
                        "${:.2}",
                        down.buyin_amount() * min_buyins(player_profile.risk_level)
                    )
                    .red()
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
                    println!(
                        "\n🎯 Target SNG Buy-In: {}",
                        format!("${:.2}", stake).truecolor(255, 215, 0).bold()
                    );
                }
                GameType::Tournament => {
                    println!(
                        "\n🎯 Target Avg MTT Buy-In: {}",
                        format!("${:.2}", stake).truecolor(255, 215, 0).bold()
                    );
                }
                _ => {}
            }
        }
    }

    println!(
        "\n{}",
        "Good luck at the tables! ♠️♥️♣️♦️".bright_white().italic()
    );
}
