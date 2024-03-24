use colored::*;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Hearts => "♥",
                Suit::Diamonds => "♦",
                Suit::Clubs => "♣",
                Suit::Spades => "♠",
            }
        )
    }
}

#[derive(Debug)]
struct Card {
    value: u8,
    suit: Suit,
}

impl Card {
    fn new(value: u8, suit: Suit) -> Result<Card, &'static str> {
        if !(1..=13).contains(&value) {
            Err("Invalid card value")
        } else {
            Ok(Card { value, suit })
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => format!("{}", self.value),
        };
        write!(f, "{}{}", value, self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in 1..=13 {
                cards.push(Card::new(value, suit).unwrap());
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

fn hand_scores(cards: &Vec<Card>) -> Vec<i8> {
    let mut total = 0;
    let mut seen_ace = false;

    for card in cards {
        total += match card.value {
            1 => {
                seen_ace = true;
                1
            }
            11..=13 => 10,
            _ => card.value as i8,
        };
    }

    let mut scores = vec![total];
    if seen_ace && total + 10 <= 21 {
        scores.push(total + 10);
    }

    scores
}

fn ask(question: &str) -> io::Result<String> {
    print!("{} ", question);
    io::stdout().flush()?;
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    Ok(answer)
}

fn read_bet_amount(max: i32) -> i32 {
    loop {
        if let Ok(input) = ask("How much would you like to bet?") {
            match input.trim().parse::<i32>() {
                Ok(n) => {
                    if n > max {
                        println!("You don't have that much money");
                        continue;
                    } else if n <= 0 {
                        continue;
                    } else {
                        return n;
                    }
                }
                Err(_) => continue,
            };
        }
    }
}

fn confirm(prompt: &str) -> bool {
    print!("{} [Y/n] ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => true,
    }
}

fn print_hand(hand: &Vec<Card>) {
    for card in hand {
        print!("{} ", card);
    }

    let player_scores = hand_scores(hand);
    print!(" score: {} ", player_scores[0]);
    if player_scores.len() >= 2 {
        print!("or {}", player_scores[1]);
    }
    println!();
}

fn main() {
    println!("{}", "$$$$$$$$$$$$$$$$$$$$$".green());
    println!("Welcome to Blackjack!");
    println!("{}", "$$$$$$$$$$$$$$$$$$$$$".green());
    println!();

    let mut deck = Deck::new();
    deck.shuffle();

    let mut money: i32 = 1000;
    println!("You have {}", format!("${}", money).green());

    loop {
        let bet = read_bet_amount(money);
        money -= bet;

        let mut player_hand = Vec::new();
        let mut dealer_hand = Vec::new();

        player_hand.push(deck.draw().unwrap());
        dealer_hand.push(deck.draw().unwrap());

        player_hand.push(deck.draw().unwrap());
        dealer_hand.push(deck.draw().unwrap());

        println!("Dealer: {} ??", dealer_hand[0]);
        print!("You: ");
        print_hand(&player_hand);

        let mut player_scores = hand_scores(&player_hand);
        let mut dealer_scores = hand_scores(&dealer_hand);

        while player_scores[0] < 21 {
            if player_scores.iter().any(|&score| score == 21) {
                break;
            }
            let answer = ask("[h]it or [s]tand?").unwrap().to_lowercase();
            if answer.starts_with('h') {
                player_hand.push(deck.draw().unwrap());
                print_hand(&player_hand);
                player_scores = hand_scores(&player_hand);
            } else if answer.starts_with('s') {
                break;
            }
        }

        if player_scores.iter().all(|&score| score > 21) {
            println!("{}", "You bust!".red());
        } else {
            // Dealer's Play
            println!("\nDealer's Play");

            while *dealer_scores.iter().max().unwrap() < 17 {
                dealer_hand.push(deck.draw().unwrap());
                dealer_scores = hand_scores(&dealer_hand);
            }
            let dealer_best_score = *dealer_scores.iter().max().unwrap();
            print!("Dealer: ");
            print_hand(&dealer_hand);

            let player_best_score = *player_scores.iter().max().unwrap();

            if dealer_best_score > 21 {
                println!("Dealer busts! {}!", "You win!".green());
                money += bet * 3 / 2;
            } else if dealer_best_score > player_best_score {
                println!("{}", "Dealer wins!".red());
            } else if dealer_best_score < player_best_score {
                println!("{}", "You win!".green());
                money += bet * 3 / 2;
            } else {
                println!("{} Bet is returned.", "Push!".yellow());
                money += bet;
            }
        }

        if money == 0 {
            println!("{}", "You're broke! Goodbye!".red().bold());
            break;
        }
        println!("\nYou have ${}", format!("{}", money).green());
        if !confirm("Do you want to continue?") {
            println!("Thanks for playing!");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hand_value() {
        let hand = vec![
            Card::new(5, Suit::Hearts).unwrap(),
            Card::new(8, Suit::Hearts).unwrap(),
        ];
        assert_eq!(hand_scores(&hand), vec![13]);
    }

    #[test]
    fn test_hand_value_order_doesnt_matter() {
        // Different order of cards should not change the result
        let hand1 = vec![
            Card::new(10, Suit::Hearts).unwrap(),
            Card::new(2, Suit::Hearts).unwrap(),
        ];
        let hand2 = vec![
            Card::new(10, Suit::Hearts).unwrap(),
            Card::new(2, Suit::Hearts).unwrap(),
        ];
        assert_eq!(hand_scores(&hand1), hand_scores(&hand2));
    }

    #[test]
    fn test_hand_value_contains_ace() {
        // First Ace has 2 values
        let hand = vec![
            Card::new(1, Suit::Hearts).unwrap(),
            Card::new(10, Suit::Hearts).unwrap(),
        ];
        assert_eq!(hand_scores(&hand), vec![11, 21]);
    }

    #[test]
    fn test_multiple_aces() {
        // Second ace only has one value
        let hand = vec![
            Card::new(5, Suit::Hearts).unwrap(),
            Card::new(1, Suit::Hearts).unwrap(),
            Card::new(1, Suit::Hearts).unwrap(),
        ];
        assert_eq!(hand_scores(&hand), vec![7, 17]);
    }

    #[test]
    fn test_hand_value_contains_ace_but_over_21() {
        let hand = vec![
            Card::new(10, Suit::Clubs).unwrap(),
            Card::new(9, Suit::Clubs).unwrap(),
            Card::new(1, Suit::Clubs).unwrap(),
        ];
        assert_eq!(hand_scores(&hand), vec![20]);
    }
}
