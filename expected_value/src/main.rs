#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Read};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use std::collections::HashMap;
//use std::collections::HashSet;
use itertools::Itertools;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::env;
use std::time::{Duration, Instant, SystemTime};
use std::cmp;
use std::{thread};
use rand::{RngCore, Rng, SeedableRng};
use rand_chacha::{ChaCha20Core,ChaCha20Rng};
use csv::Writer;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
enum CardSuit {
  Heart = 1,
  Spade = 2,
  Club = 3,
  Diamond = 4,
}

impl fmt::Display for CardSuit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        CardSuit::Heart => write!(f, "h"),
        CardSuit::Spade => write!(f, "s"),
        CardSuit::Club => write!(f, "c"),
        CardSuit::Diamond => write!(f, "d"),
      }
  }
}

impl fmt::Debug for CardSuit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        CardSuit::Heart => write!(f, "h"),
        CardSuit::Spade => write!(f, "s"),
        CardSuit::Club => write!(f, "c"),
        CardSuit::Diamond => write!(f, "d"),
      }
  }
}

#[derive(Hash, Clone, Copy, Serialize, Deserialize)]
struct Card {
  rank: u8, // [2..14]
  suit: CardSuit,
}

impl Ord for Card {
  fn cmp(&self, other: &Self) -> Ordering {
    let rank_ord = self.rank.cmp(&other.rank);
    if rank_ord == Ordering::Equal {
      let s1 = self.suit as u8;
      let s2 = other.suit as u8;
      return s1.cmp(&s2);
    } else {
      return rank_ord
    }
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for Card {
  fn eq(&self, other: &Self) -> bool {
      self.rank == other.rank && self.suit == other.suit
  }
}
impl Eq for Card {}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let rank = match self.rank {
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "T",
        11 => "J",
        12 => "Q",
        13 => "K",
        14 => "A",
        _ => panic!("unknown rank: {}", self.rank),
      };
      write!(f, "{}{}", rank, self.suit)
  }
}

impl fmt::Debug for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let rank = match self.rank {
      2 => "2",
      3 => "3",
      4 => "4",
      5 => "5",
      6 => "6",
      7 => "7",
      8 => "8",
      9 => "9",
      10 => "T",
      11 => "J",
      12 => "Q",
      13 => "K",
      14 => "A",
      _ => panic!("unknown rank: {}", self.rank),
    };
    write!(f, "{}{}", rank, self.suit)
  }
}

impl FromStr for Card {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (suit_str, rank_str) = s.split_at(1);
    let rank;
    let suit;
    // we can have two notations for card:
    // H14 or Ah. H10 or 10h
    if suit_str == "H" || suit_str == "S" || suit_str == "C" || suit_str == "D" {
      rank = rank_str.parse::<u8>()?;
      suit = match suit_str {
        "H"|"h" => CardSuit::Heart,
        "S"|"s" => CardSuit::Spade,
        "C"|"c" => CardSuit::Club,
        "D"|"d" => CardSuit::Diamond,
        _ => panic!("unknown suit"),
      };
    } else {
      let (rank_str, suit_str) = s.split_at(s.len()-1);
      rank = match rank_str {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "T"|"10" => 10,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => panic!("unknown rank: {}", rank_str),
      };
      suit = match suit_str {
        "H"|"h" => CardSuit::Heart,
        "S"|"s" => CardSuit::Spade,
        "C"|"c" => CardSuit::Club,
        "D"|"d" => CardSuit::Diamond,
        _ => panic!("unknown suit"),
      };
    }
    
    Ok(Card{rank: rank, suit: suit})
  }
}

struct HandData {
  cards: Vec<Card>,
  value: f32,
}

//example: ['H2' 'H3' 'H4' 'H5' 'S7'],7.05432
impl FromStr for HandData {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let str_parts: Vec<&str> = s.split(',').collect();
    let hand_str = str_parts[0];
    let value_str = str_parts[1];
    let hand_str2 = hand_str.trim_matches(|c| c == '[' || c == ']');
    let hand_parts: Vec<&str> = hand_str2.split(' ').collect();
    let mut cards = Vec::new();
    for h in hand_parts {
      let h_trimmed = h.trim_matches('\'');
      let card = h_trimmed.parse::<Card>().unwrap();
      cards.push(card);
    }

    let val = value_str.parse::<f32>().unwrap();

    Ok(HandData { cards: cards, value: val })
  }
}

impl fmt::Display for HandData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "[{} {} {} {} {}],{}", self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.value)
  }
}

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
enum HandRank {
  HighCard = 1,
  Pair = 2,
  TwoPairs = 3,
  ThreeOfAKind = 4,
  Straight = 5,
  Flush = 6,
  FullHouse = 7,
  FourOfAKind = 8,
  StraightFlush = 9,
  RoyalFlush = 10,
}

impl fmt::Display for HandRank {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      HandRank::HighCard      => write!(f, "0/9 High card"),
      HandRank::Pair          => write!(f, "1/9 Pair"),
      HandRank::TwoPairs      => write!(f, "2/9 Two Pairs"),
      HandRank::ThreeOfAKind  => write!(f, "3/9 Three of a Kind"),
      HandRank::Straight      => write!(f, "4/9 Straight"),
      HandRank::Flush         => write!(f, "5/9 Flush"),
      HandRank::FullHouse     => write!(f, "6/9 Full House"),
      HandRank::FourOfAKind   => write!(f, "7/9 Four of a Kind"),
      HandRank::StraightFlush => write!(f, "8/9 Straight Flush"),
      HandRank::RoyalFlush    => write!(f, "9/9 Royal Flush"),
    }
  }
}

impl fmt::Debug for HandRank {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      HandRank::HighCard      => write!(f, "0/9 High card"),
      HandRank::Pair          => write!(f, "1/9 Pair"),
      HandRank::TwoPairs      => write!(f, "2/9 Two Pairs"),
      HandRank::ThreeOfAKind  => write!(f, "3/9 Three of a Kind"),
      HandRank::Straight      => write!(f, "4/9 Straight"),
      HandRank::Flush         => write!(f, "5/9 Flush"),
      HandRank::FullHouse     => write!(f, "6/9 Full House"),
      HandRank::FourOfAKind   => write!(f, "7/9 Four of a Kind"),
      HandRank::StraightFlush => write!(f, "8/9 Straight Flush"),
      HandRank::RoyalFlush    => write!(f, "9/9 Royal Flush"),
    }
  }
}

fn read<R: Read>(io: R) -> Result<Vec<HandData>, Error> {
  let br = BufReader::new(io);
  br.lines()
      .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
      .collect()
}

fn conv_string_to_cards(s: &str) -> Vec<Card> {
  let parts: Vec<&str> = s.split(' ').collect();
  let mut res = Vec::new();
  for p in parts {
    if p == "Empty" {
      continue;
    }
    let card = p.parse::<Card>().unwrap();
    res.push(card);
  }
  return res
}

fn find_common_cards_in_pack(cards: &Vec<&Card>, pack: &Vec<Card>) -> usize {
  let mut common = 0;
  for i in 0..cards.len() {
    for j in 0..pack.len() {
      if (*cards[i]) == pack[j] {
        common = common+1;
        break;
      }
    }
  }

  return common
}

fn find_common_cards_in_pack_no_ref(cards: &Vec<Card>, pack: &Vec<Card>) -> usize {
  let mut common = 0;
  for i in 0..cards.len() {
    for j in 0..pack.len() {
      if (cards[i]) == pack[j] {
        common = common+1;
        break;
      }
    }
  }

  return common
}

fn find_possible_hands_in_all_combinations(cards: &Vec<&Card>, combinations: &HashMap<Vec<Card>,(f32,f32)>) -> Vec<Vec<Card>> {
  let mut possible_hands = Vec::new();
  for (k, _) in combinations {
    if find_common_cards_in_pack(cards, k) == cards.len() {
      possible_hands.push(k.to_vec());
    }
  }
  return possible_hands
}

fn find_possible_hands_in_all_combinations_no_ref(cards: &Vec<Card>, combinations: &HashMap<Vec<Card>,(f32,f32)>) -> Vec<Vec<Card>> {
  let mut possible_hands = Vec::new();
  for (k, _) in combinations {
    if find_common_cards_in_pack_no_ref(cards, k) == cards.len() {
      possible_hands.push(k.to_vec());
    }
  }
  return possible_hands
}

#[allow(illegal_floating_point_literal_pattern)]
fn get_best_hand_string(f: f32) -> HandRank {
  return match f {
    0.0..=99.999 =>    HandRank::HighCard,
    100.0..=199.999 => HandRank::Pair,
    200.0..=299.999 => HandRank::TwoPairs,
    300.0..=399.999 => HandRank::ThreeOfAKind,
    400.0..=499.999 => HandRank::Straight,
    500.0..=599.999 => HandRank::Flush,
    600.0..=699.999 => HandRank::FullHouse,
    700.0..=799.999 => HandRank::FourOfAKind,
    800.0..=899.999 => HandRank::StraightFlush,
    900.0..=999.999 => HandRank::RoyalFlush,
    _ => panic!("unknown range for {}", f),
  };
}

fn is_hand_part_of_made_up_hand(hand: &Vec<Card>, hand_rank: &HandRank, combination: &Vec<Card>) -> bool {
  let mut community_cards = combination.clone();
  community_cards.retain(|&x| x != hand[0]);
  community_cards.retain(|&x| x != hand[1]);

  return match hand_rank {
    HandRank::HighCard => {
      true
    },
    HandRank::Pair => {
      for i in 0..hand.len() {
        for j in 0..community_cards.len() {
          if hand[i].rank == community_cards[j].rank {
            return true
          }
        }
      }
      false
    },
    HandRank::TwoPairs => {
      let mut num_matched = 0;
      for i in 0..hand.len() {
        for j in 0..community_cards.len() {
          if hand[i].rank == community_cards[j].rank {
            num_matched = num_matched+1;
          }
        }
      }
      num_matched == 2 // Kd Ad vs 7h 7d Ah <- will skip this case, intentionally
    },
    HandRank::ThreeOfAKind => {
      let mut num_matched = 0;
      for i in 0..hand.len() {
        for j in 0..community_cards.len() {
          if hand[i].rank == community_cards[j].rank {
            num_matched = num_matched+1;
          }
        }
      }
      num_matched == 2 // 2 is correct here. 2,2 vs 2,5,6. or 2,3 vs 2,2,6
    },
    HandRank::Straight => {
      true
    },
    HandRank::Flush => {
      true
    },
    HandRank::FullHouse => {
      true
    },
    HandRank::FourOfAKind => {
      true
    },
    HandRank::StraightFlush => {
      true
    },
    HandRank::RoyalFlush => {
      true
    },
  }
}

// returns tuple of: raw hand value, hand equity, type of hand
fn get_best_hand(my_hand: &Vec<Card>, community: &Vec<Card>, combinations: &HashMap<Vec<Card>,(f32,f32)>) -> (f32, f32, HandRank, Vec<Card>) {
  let mut sorted_cards = Vec::<Card>::new();
  for h in my_hand {
    sorted_cards.push(*h);
  }
  for h in community {
    sorted_cards.push(*h);
  }
  sorted_cards.sort();
  let mut assembled_hand = Vec::<Card>::new();
  let (highest_value, equity) = match sorted_cards.len() {
    5 =>  {
      let highest_score;
      let highest_eq;
      let (score, eq) = combinations[&sorted_cards];
      assembled_hand = sorted_cards.clone();
      highest_score = score;
      highest_eq = eq;
    (highest_score, highest_eq)
    },
    6|7 => {
      let possible_hands = sorted_cards.iter().combinations(5);
      let mut highest_score = 0.0;
      let mut highest_eq = 0.0;
      for hand in possible_hands {
        let mut new_hand = Vec::<Card>::new();
        for c in hand {
          new_hand.push(*c);
        }
        new_hand.sort();
        if find_common_cards_in_pack_no_ref(my_hand, &new_hand) == 0 {
          continue;
        }
        let (score, eq) = combinations[&new_hand];
        if score > highest_score {
          highest_score = score;
          highest_eq = eq;
          assembled_hand = new_hand.clone();
        }
      }
      (highest_score, highest_eq)
    },
    _ => panic!("unexpected cards len {} in get_best_hand", sorted_cards.len()),
  };
  return (highest_value, equity, get_best_hand_string(highest_value), assembled_hand)
}

fn calculcate_hand_ev(input: &str, pot_str: &str, card_deck: &Vec<Card>, starting_hands: &HashMap<Vec<Card>, (f32,f32,f32)>, combinations: &HashMap<Vec<Card>, (f32,f32)>, simulated_hands: &HashMap::<Vec<Card>, (u64, u64, HashMap<HandRank, u64>, u64, u64, u64)>) {
  //let start_main_ts = Instant::now();
  let mut total_pot = 0.0;
  let mut main_pot = 0.0;
  if pot_str.contains('$') {
    let split_pot_str: Vec<&str> = pot_str.split('\n').collect();
    for s in split_pot_str {
      let semicolon = s.find(':');
      if semicolon != None {
        let (s1, s2) = s.split_at(semicolon.unwrap());
        let pot_name = s1.to_lowercase();
        if pot_name == "total pot" {
          let (_t, am) = s2.split_at(3);
          total_pot = am.parse::<f32>().unwrap();
        } else if pot_name == "main pot" {
          let (_t, am) = s2.split_at(3);
          main_pot = am.parse::<f32>().unwrap();
        }
      }      
    }
  }
  let call_amount = total_pot-main_pot;
  //println!("{}, {}", total_pot, main_pot);

  let mut input_cards = conv_string_to_cards(&input);

  // check that input cards don't have duplicates in case if ML messed up recognizing cards
  let mut has_duplicate_cards = false;
  for i in 0..input_cards.len() {
    for j in (i+1)..input_cards.len() {
      if input_cards[i] == input_cards[j] {
        has_duplicate_cards = true;
        break;
      }
    }
  }
  if has_duplicate_cards {
    println!("Detected duplicate card in input: {:?}", input_cards);
    return
  }

  if input_cards.len() == 2 {
    input_cards.sort();
    let (_, avg_eq, _) = starting_hands[&input_cards];
    /*let action;
    if start_eq < 0.5 {
      action = "FOLD";
    } else {
      action = "CALL";
    }*/
    println!("hand cards: {:?}, Eq: {:.2}%", input_cards, avg_eq*100.0);

    {
      let (num_won, num_total, _, won_flop, won_turn, won_river) = simulated_hands[&input_cards];
      let win_ch = num_won as f64/num_total as f64;
      println!("SimData: {:?} - win: {:.2}%, flop: {:.2}% turn: {:.2}% river: {:.2}%", input_cards, win_ch*100.0, (won_flop as f64/num_won as f64)*100.0, (won_turn as f64/num_won as f64)*100.0, (won_river as f64/num_won as f64)*100.0);
    }
  
    return
  }
  if input_cards.len() < 5 {
    println!("not enough cards, only got: {:?}", input_cards);
    return
  }

  let mut hand = Vec::<Card>::new();
  let mut community = Vec::<Card>::new();
  hand.push(input_cards[0]);
  hand.push(input_cards[1]);
  hand.sort();
  for i in 2..input_cards.len() {
    community.push(input_cards[i]);
  }

  println!("hand cards: {:?}", hand);
  println!("community cards: {:?}", community);
  //println!("Pot: ${:.2}, To Call: ${:.2}", total_pot, call_amount);

  let mut all_cards = Vec::<Card>::new();
  all_cards.extend(hand.to_vec().iter());
  all_cards.extend(community.to_vec().iter());

  let (flop_hand_type, real_my_hand_eq, improved_hands_hash_map, opponent_hands_hash_map, opponent_num_hands) 
    = get_hand_equity_and_opponent_range(&hand, &community, &combinations, &starting_hands, &card_deck);
  let num_cards_in_deck_left = (card_deck.len()-community.len()-hand.len()) as i32;

  //println!("Oppont: {:.2}%", oppon_eq*100.0);
  //println!("{:.3}-{:.3}", min_eq, max_eq);
  println!("Opponent hand range:");
  let mut sorted_keys: Vec<&HandRank> = opponent_hands_hash_map.keys().collect();
  sorted_keys.sort();
  for hand_type in sorted_keys {
    if *hand_type == HandRank::HighCard {
      continue;
    }
    let s = hand_type.to_string();
    println!("{:<20}:{:.1}%", s, (opponent_hands_hash_map[&hand_type] as f32/opponent_num_hands as f32)*100.0);
  }

  // show my hands relative strength to any opponent's hand. essentially it is my equity
  println!("Hand Equity: {:.2}%, Type: {}", real_my_hand_eq*100.0, flop_hand_type);
  let mut sorted_keys: Vec<&HandRank> = improved_hands_hash_map.keys().collect();
  sorted_keys.sort();
  for hand_type in sorted_keys {
    let s = hand_type.to_string();
    let num_outs = improved_hands_hash_map[hand_type];
    let perc = num_outs as f32/(num_cards_in_deck_left-num_outs) as f32;
    // todo: to make proper call calc need to add info about how many players are betting
    let new_total_pot = total_pot+call_amount; // this is if I add my call to the pot
    let pot_perc;
    if new_total_pot > 0.0 && call_amount > 0.0 {
      pot_perc = call_amount / new_total_pot;
    } else {
      pot_perc = 0.0;
    }
    let action;
    if perc > pot_perc {
      action = "CALL";
    } else {
      action = "BAD CALL";
    }
    if pot_perc == 0.0 {
      println!("{:<20}:{:.1}%", s, perc*100.0);
    } else {
      println!("{:<20}:{:.1}%  pot odds: {:.1}%, action: {}", s, perc*100.0, pot_perc*100.0, action);
    }
  }

  {
    let (num_won, num_total, _, won_flop, won_turn, won_river) = simulated_hands[&hand];
    let win_ch = num_won as f64/num_total as f64;
    println!("SimData: {:?} - win: {:.2}%, flop: {:.2}% turn: {:.2}% river: {:.2}%", hand, win_ch*100.0, (won_flop as f64/num_won as f64)*100.0, (won_turn as f64/num_won as f64)*100.0, (won_river as f64/num_won as f64)*100.0);
  }

  //let duration_main = start_main_ts.elapsed();
  //println!("Main duration is: {:?}", duration_main);
}

fn main() -> Result<(), Error> {
  //let start_init_ts = Instant::now();
  let combinations_path: String =   "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/combinations.bin".to_string();
  let starting_hands_path: String = "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/starting_hands.bin".to_string();
  let simulated_hands_path: String = "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/simulated_hands.bin".to_string();
  let hands_csv_path: String =      "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/hands.csv".to_string();
  let trigger_path: String =        "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/trigger".to_string();
  let input_hand_path: String =     "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/input_hand".to_string();
  let input_pot_path: String =      "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/input_pot".to_string();

  let card_deck = conv_string_to_cards("2c 3c 4c 5c 6c 7c 8c 9c Tc Jc Qc Kc Ac 2h 3h 4h 5h 6h 7h 8h 9h Th Jh Qh Kh Ah 2s 3s 4s 5s 6s 7s 8s 9s Ts Js Qs Ks As 2d 3d 4d 5d 6d 7d 8d 9d Td Jd Qd Kd Ad");

  let mut combinations = HashMap::new();
  let mut starting_hands = HashMap::new();
  if Path::new(&combinations_path).exists() {
    let f = BufReader::new(File::open(combinations_path).unwrap());
    combinations = bincode::deserialize_from(f).unwrap();
  } else {
    println!("Generating combinations...");
    let vec = read(File::open(hands_csv_path)?)?;
    for i in 0..vec.len() {
      let v = &vec[i];
      let equity = i as f32/vec.len() as f32;
      //println!("{}: {}, {}", i, v.value, equity);
      // key is hand set. value is (raw value of the card, hand equity as chance to win with that hand)
      combinations.insert(v.cards.to_vec(), (v.value, equity));
    }
  
    let mut f = BufWriter::new(File::create(combinations_path).unwrap());
    bincode::serialize_into(&mut f, &combinations).unwrap();
  }

  if Path::new(&starting_hands_path).exists() {
    let f = BufReader::new(File::open(starting_hands_path).unwrap());
    starting_hands = bincode::deserialize_from(f).unwrap();
  } else {
    println!("Generating starting hands...");
    for i in 0..card_deck.len() {
      for j in (i+1)..card_deck.len() {
        let mut hand = vec![card_deck[i], card_deck[j]];
        hand.sort();
        let found_hands = find_possible_hands_in_all_combinations_no_ref(&hand, &combinations);
        let mut total_eq = 0.0;
        let mut min_eq = 100.0;
        let mut max_eq = 0.0;
        let mut num_hands = 0;
        for h in found_hands {
          let (hscore, hequity) = combinations[&h];
          let hand_rank = get_best_hand_string(hscore);
          // check if hand is actually part of winning hand
          if is_hand_part_of_made_up_hand(&hand, &hand_rank, &h) {
            num_hands = num_hands+1;
            total_eq = total_eq + hequity;
            if hequity < min_eq {
              min_eq = hequity;
            }
            if hequity > max_eq {
              max_eq = hequity;
            }
          } else {
            //println!("skipping incorrect hand: {:?}, {:?}, {}", hand, h, hand_rank);
          }
        }
        let aver_eq = total_eq/num_hands as f32;
        //println!("starting hand: {:?} - {:.2}%", hand, aver_eq*100.0);

        starting_hands.insert(hand, (min_eq, aver_eq, max_eq));
      }
    }

    let mut f = BufWriter::new(File::create(starting_hands_path).unwrap());
    bincode::serialize_into(&mut f, &starting_hands).unwrap();
  }

  // hash map of starting hand and tuple of 
  //      (number of games won, total number of games seen, hand rank of this game, 
  //      number of times this hand was winning hand starting with preflop,
  //      number of times this hand become winning on the turn,
  //      number of times this hand become winning on the river)
  let mut simulated_hands = HashMap::<Vec<Card>, (u64, u64, HashMap<HandRank, u64>, u64, u64, u64)>::new();
  if Path::new(&simulated_hands_path).exists() {
    let f = BufReader::new(File::open(simulated_hands_path).unwrap());
    simulated_hands = bincode::deserialize_from(f).unwrap();
  } else {
    println!("Generating simulated hands...");
    simulate_game(10, 10000000, 6, &mut simulated_hands, &combinations, &card_deck);

    let mut f = BufWriter::new(File::create(simulated_hands_path).unwrap());
    bincode::serialize_into(&mut f, &simulated_hands).unwrap();
  }
  
  /*let mut test_hand = conv_string_to_cards("Ks 8s");
  test_hand.sort();
  let (num_won, num_total, _) = simulated_hands[&test_hand];
  println!("{:.2}% - {:?}", (num_won as f64/num_total as f64)*100.0, simulated_hands[&test_hand].2);
  */

  /*let mut hands: Vec<&Vec<Card>> = simulated_hands.keys().collect();
  hands.sort();
  for h in hands {
    let (num_won, num_total, _, won_flop, won_turn, won_river) = simulated_hands[h];
    let win_ch = num_won as f64/num_total as f64;
    if win_ch > 0.2 {
      println!("{:?} - {:.2}%, flop: {:.2}% turn: {:.2}% river: {:.2}%", h, win_ch*100.0, (won_flop as f64/num_won as f64)*100.0, (won_turn as f64/num_won as f64)*100.0, (won_river as f64/num_won as f64)*100.0);
    }
  }*/

  // generate ml data
  //generate_ml_data(10, 1000000, 6, &simulated_hands, &combinations, &card_deck, &starting_hands);

  //let duration_init = start_init_ts.elapsed();
  //println!("Init duration is: {:?}", duration_init);

  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Usage: poker_ev mode input_hand pot");
    return Ok(())
  }
  let mode: &str = &(args[1]);
  match mode {
    "once" => {
      if args.len() != 4 {
        panic!("Not enough arguments provided. Expecting mode input_hand pot");
      }
      // example hand input: "C8 H5 H7 D12 D6"
      // example put input: "Total pot: $1.30\nMain pot: $1.10\n\n"
      calculcate_hand_ev(&(args[2]), &(args[3]), &card_deck, &starting_hands, &combinations, &simulated_hands);
    },
    "loop" => {
      let trigger_path_file = Path::new(&trigger_path);
      loop {
        if trigger_path_file.exists() {
          let input_hand = fs::read_to_string(Path::new(&input_hand_path)).unwrap();
          let input_hand2 = input_hand.trim();
          let input_pot = fs::read_to_string(Path::new(&input_pot_path)).unwrap();
          fs::remove_file(trigger_path_file).unwrap();

          calculcate_hand_ev(&input_hand2, &input_pot, &card_deck, &starting_hands, &combinations, &simulated_hands);
          println!("END");
        } else {
          let sleep_amount = Duration::from_millis(100);
          thread::sleep(sleep_amount);
        }        
      }
    }
    _ => panic!("unknown mode: {}", mode),
  };

  Ok(())
}

fn simulate_game(outter_runs: u32, max_sim_runs: u64, num_pl: usize, simulated_hands: &mut HashMap::<Vec<Card>, (u64, u64, HashMap<HandRank, u64>, u64, u64, u64)>, combinations: &HashMap<Vec<Card>,(f32,f32)>, card_deck: &Vec<Card>) {

  let mut rng = ChaCha20Rng::seed_from_u64(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

  let num_cards_in_deck = card_deck.len() as u32;

  for outter_run in 0..outter_runs {
    for sim_run_iter in 0..max_sim_runs {
      println!("Running sim {}:{}/{}", outter_run+1, sim_run_iter+1, max_sim_runs);

      let mut new_deck = card_deck.clone();
      // shuffle cards
      for ctr in 0..new_deck.len() {
        let random_number = (rng.next_u32() % num_cards_in_deck) as usize;
        let tmp = new_deck[random_number];
        new_deck[random_number] = new_deck[ctr];
        new_deck[ctr] = tmp;
      }

      let mut players = Vec::<(Vec<Card>,(f32, HandRank, bool, bool, bool))>::new();
      players.resize(num_pl, (Vec::<Card>::new(),(0.0, HandRank::HighCard, false, false, false)));
      for i in 0..num_pl {
        players[i].0.push(new_deck.pop().unwrap());
      }
      for i in 0..num_pl {
        players[i].0.push(new_deck.pop().unwrap());
        players[i].0.sort();
      }
      //println!("{:?}", players);
      let mut community_cards = Vec::<Card>::new();
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      //println!("{:?}", community_cards);
      for i in 0..num_pl {
        let (_, hand_equity, hand_rank, _) = get_best_hand(&players[i].0, &community_cards, &combinations);
        players[i].1 = (hand_equity, hand_rank, false, false, false);
      }
      //println!("{:?}", players);
      players.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
      //println!("{:?}", players);
      //now test if hand was winning starting with the flop
      let mut temp_players = players.clone();
      let mut flop_only = community_cards.clone();
      flop_only.pop();
      flop_only.pop();
      for i in 0..num_pl {
        let (_, hand_equity, hand_rank, _) = get_best_hand(&players[i].0, &flop_only, &combinations);
        temp_players[i].1 = (hand_equity, hand_rank, false, false, false);
      }
      temp_players.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
      // now see if top player was top on the flop
      if temp_players[0].0 == players[0].0 {
        players[0].1.2 = true;
      }

      //now test if hand was winning starting with the turn
      let mut temp_players = players.clone();
      let mut flop_and_turn_only = community_cards.clone();
      flop_and_turn_only.pop();
      for i in 0..num_pl {
        let (_, hand_equity, hand_rank, _) = get_best_hand(&players[i].0, &flop_and_turn_only, &combinations);
        temp_players[i].1 = (hand_equity, hand_rank, false, false, false);
      }
      temp_players.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
      // now see if top player was top on the turn and was not already at the top on flop
      if temp_players[0].0 == players[0].0 && players[0].1.2 == false {
        players[0].1.3 = true;
      }

      // and now test if top player become top only on the river
      if players[0].1.2 == false && players[0].1.3 == false {
        players[0].1.4 = true;
      }


      for i in 0..num_pl {
        let (_, hand_rank, win_flop, win_turn, win_river) = players[i].1;
        if simulated_hands.contains_key(&players[i].0) {
          let (mut num_wons, mut num_games, _, num_won_flop, num_won_turn, num_won_river) = simulated_hands[&players[i].0];
          if i == 0 {
            num_wons = num_wons+1;
          }
          num_games = num_games+1;
          *(simulated_hands.get_mut(&players[i].0).unwrap().2).entry(hand_rank).or_insert(1) += 1;
          simulated_hands.get_mut(&players[i].0).unwrap().0 = num_wons;
          simulated_hands.get_mut(&players[i].0).unwrap().1 = num_games;
          if win_flop {
            simulated_hands.get_mut(&players[i].0).unwrap().3 = num_won_flop+1;
          }
          if win_turn {
            simulated_hands.get_mut(&players[i].0).unwrap().4 = num_won_turn+1;
          }
          if win_river {
            simulated_hands.get_mut(&players[i].0).unwrap().5 = num_won_river+1;
          }
        } else {
          let mut ranks = HashMap::<HandRank, u64>::new();
          ranks.insert(hand_rank, 1);
          if i == 0 {
            simulated_hands.insert(players[i].0.clone(), (1,1, ranks, win_flop as u64, win_turn as u64, win_river as u64));
          } else {
            simulated_hands.insert(players[i].0.clone(), (0,1, ranks, win_flop as u64, win_turn as u64, win_river as u64));
          }
        }
      }
      //println!("{:?}", simulated_hands);
    }
  }
}

fn convert_card_to_int(card: &Card) -> u32 {
  match card.suit {
    CardSuit::Heart => (card.rank as u32)-1, // [2..14] -> [1..13]
    CardSuit::Spade => 13+((card.rank as u32)-1), // [2..14] -> [14..26]
    CardSuit::Club => 26+((card.rank as u32)-1), 
    CardSuit::Diamond => 39+((card.rank as u32)-1), 
  }
}

fn generate_ml_data(outter_runs: u32, max_sim_runs: u64, num_pl: usize, simulated_hands: &HashMap::<Vec<Card>, (u64, u64, HashMap<HandRank, u64>, u64, u64, u64)>, combinations: &HashMap<Vec<Card>,(f32,f32)>, card_deck: &Vec<Card>, starting_hands: &HashMap<Vec<Card>, (f32,f32,f32)>) {
  let ml_data_path = "/home/pavel/nvme/GitHub/poker_ml/expected_value/data/ml_data.csv".to_string();
  let mut csv_writer = Writer::from_path(ml_data_path).unwrap();

  let mut rng = ChaCha20Rng::seed_from_u64(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

  let num_cards_in_deck = card_deck.len() as u32;

  // write header
  csv_writer.write_field("state").unwrap();
  csv_writer.write_field("hand1").unwrap();
  csv_writer.write_field("hand2").unwrap();
  csv_writer.write_field("flop1").unwrap();
  csv_writer.write_field("flop2").unwrap();
  csv_writer.write_field("flop3").unwrap();
  csv_writer.write_field("turn").unwrap();
  csv_writer.write_field("river").unwrap();
  csv_writer.write_field("win_chance").unwrap();
  csv_writer.write_field("won_on_flop").unwrap();
  csv_writer.write_field("won_on_turn").unwrap();
  csv_writer.write_field("won_on_river").unwrap();
  csv_writer.write_field("hand_equity").unwrap();
  csv_writer.write_field("did_win").unwrap();
  csv_writer.write_record(None::<&[u8]>).unwrap();  

  for outter_run in 0..outter_runs {
    for sim_run_iter in 0..max_sim_runs {
      println!("Running sim {}:{}/{}", outter_run+1, sim_run_iter+1, max_sim_runs);

      let mut new_deck = card_deck.clone();
      // shuffle cards
      for ctr in 0..new_deck.len() {
        let random_number = (rng.next_u32() % num_cards_in_deck) as usize;
        let tmp = new_deck[random_number];
        new_deck[random_number] = new_deck[ctr];
        new_deck[ctr] = tmp;
      }

      let mut players = Vec::<(Vec<Card>,(f32, HandRank))>::new();
      players.resize(num_pl, (Vec::<Card>::new(),(0.0, HandRank::HighCard)));
      for i in 0..num_pl {
        players[i].0.push(new_deck.pop().unwrap());
      }
      for i in 0..num_pl {
        players[i].0.push(new_deck.pop().unwrap());
        players[i].0.sort();
      }
      //println!("{:?}", players);
      let mut community_cards = Vec::<Card>::new();
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      community_cards.push(new_deck.pop().unwrap());
      //println!("{:?}", community_cards);
      for i in 0..num_pl {
        let (_, hand_equity, hand_rank, _) = get_best_hand(&players[i].0, &community_cards, &combinations);
        players[i].1 = (hand_equity, hand_rank);
      }
      //println!("{:?}", players);
      players.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

      let my_player_idx = (rng.next_u32() % (num_pl as u32)) as usize;
      let my_player_won;
      if my_player_idx == 0 {
        my_player_won = 1;
      } else {
        my_player_won = 0;
      }
      // PRE-FLOP
      csv_writer.write_field("PREFLOP").unwrap();
      csv_writer.write_field(&players[my_player_idx].0[0].to_string()).unwrap();
      csv_writer.write_field(&players[my_player_idx].0[1].to_string()).unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field("none").unwrap();
      let (num_won, num_total, _, won_flop, won_turn, won_river) = simulated_hands[&players[my_player_idx].0];
      let win_ch = num_won as f64/num_total as f64;
      csv_writer.write_field(win_ch.to_string()).unwrap();
      let won_on_flop = won_flop as f64/num_won as f64;
      csv_writer.write_field(won_on_flop.to_string()).unwrap();
      let won_on_turn = won_turn as f64/num_won as f64;
      csv_writer.write_field(won_on_turn.to_string()).unwrap();
      let won_on_river = won_river as f64/num_won as f64;
      csv_writer.write_field(won_on_river.to_string()).unwrap();
      let hand_eq = starting_hands[&players[my_player_idx].0].1;
      csv_writer.write_field(hand_eq.to_string()).unwrap();
      csv_writer.write_field(my_player_won.to_string()).unwrap();
      csv_writer.write_record(None::<&[u8]>).unwrap();
      // FLOP
      csv_writer.write_field("FLOP").unwrap();
      csv_writer.write_field(&players[my_player_idx].0[0].to_string()).unwrap();
      csv_writer.write_field(&players[my_player_idx].0[1].to_string()).unwrap();
      csv_writer.write_field(&community_cards[0].to_string()).unwrap();
      csv_writer.write_field(&community_cards[1].to_string()).unwrap();
      csv_writer.write_field(&community_cards[2].to_string()).unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field(win_ch.to_string()).unwrap();
      csv_writer.write_field(won_on_flop.to_string()).unwrap();
      csv_writer.write_field(won_on_turn.to_string()).unwrap();
      csv_writer.write_field(won_on_river.to_string()).unwrap();
      let mut flop_cards = community_cards.clone();
      flop_cards.pop();
      flop_cards.pop();
      let (_, real_my_hand_eq, _, _, _) = get_hand_equity_and_opponent_range(&players[my_player_idx].0, &flop_cards, &combinations, &starting_hands, &card_deck);
      csv_writer.write_field(real_my_hand_eq.to_string()).unwrap();
      csv_writer.write_field(my_player_won.to_string()).unwrap();
      csv_writer.write_record(None::<&[u8]>).unwrap();
      // TURN
      csv_writer.write_field("TURN").unwrap();
      csv_writer.write_field(&players[my_player_idx].0[0].to_string()).unwrap();
      csv_writer.write_field(&players[my_player_idx].0[1].to_string()).unwrap();
      csv_writer.write_field(&community_cards[0].to_string()).unwrap();
      csv_writer.write_field(&community_cards[1].to_string()).unwrap();
      csv_writer.write_field(&community_cards[2].to_string()).unwrap();
      csv_writer.write_field(&community_cards[3].to_string()).unwrap();
      csv_writer.write_field("none").unwrap();
      csv_writer.write_field(win_ch.to_string()).unwrap();
      csv_writer.write_field(won_on_flop.to_string()).unwrap();
      csv_writer.write_field(won_on_turn.to_string()).unwrap();
      csv_writer.write_field(won_on_river.to_string()).unwrap();
      let mut flop_cards = community_cards.clone();
      flop_cards.pop();
      let (_, real_my_hand_eq, _, _, _) = get_hand_equity_and_opponent_range(&players[my_player_idx].0, &flop_cards, &combinations, &starting_hands, &card_deck);
      csv_writer.write_field(real_my_hand_eq.to_string()).unwrap();
      csv_writer.write_field(my_player_won.to_string()).unwrap();
      csv_writer.write_record(None::<&[u8]>).unwrap();
      // RIVER
      csv_writer.write_field("RIVER").unwrap();
      csv_writer.write_field(&players[my_player_idx].0[0].to_string()).unwrap();
      csv_writer.write_field(&players[my_player_idx].0[1].to_string()).unwrap();
      csv_writer.write_field(&community_cards[0].to_string()).unwrap();
      csv_writer.write_field(&community_cards[1].to_string()).unwrap();
      csv_writer.write_field(&community_cards[2].to_string()).unwrap();
      csv_writer.write_field(&community_cards[3].to_string()).unwrap();
      csv_writer.write_field(&community_cards[4].to_string()).unwrap();
      csv_writer.write_field(win_ch.to_string()).unwrap();
      csv_writer.write_field(won_on_flop.to_string()).unwrap();
      csv_writer.write_field(won_on_turn.to_string()).unwrap();
      csv_writer.write_field(won_on_river.to_string()).unwrap();
      let (_, real_my_hand_eq, _, _, _) = get_hand_equity_and_opponent_range(&players[my_player_idx].0, &community_cards, &combinations, &starting_hands, &card_deck);
      csv_writer.write_field(real_my_hand_eq.to_string()).unwrap();
      csv_writer.write_field(my_player_won.to_string()).unwrap();
      csv_writer.write_record(None::<&[u8]>).unwrap();
     
      csv_writer.flush().unwrap();
    }
  }
}

fn get_hand_equity_and_opponent_range(
  hand: &Vec<Card>, community: &Vec<Card>, combinations: &HashMap<Vec<Card>, (f32,f32)>,
  starting_hands: &HashMap<Vec<Card>, (f32,f32,f32)>, card_deck: &Vec<Card>
) -> (HandRank, f32, HashMap<HandRank, i32>, HashMap<HandRank, i32>, i32) {
    
  let (_, flop_equity, flop_hand_type, _) = get_best_hand(&hand, &community, &combinations);

  let community_cards = community.clone();
  let num_comm_cards = community_cards.len();
  // lets find cards that can improve our current hand
  let mut remaining_deck = card_deck.to_vec();
  for i in 0..community_cards.len() {
    remaining_deck.retain(|&x| x != community_cards[i]);
  }
  remaining_deck.retain(|&x| x != hand[0]);
  remaining_deck.retain(|&x| x != hand[1]);
  let mut improved_hands_hash_map = HashMap::new();
  if num_comm_cards == 3 || num_comm_cards == 4 {
    for i in 0..remaining_deck.len() {
      let mut new_comm_cards = community_cards.clone();
      new_comm_cards.push(remaining_deck[i]);
      let (_, _, htype, assembled_hand) = get_best_hand(&hand, &new_comm_cards, &combinations);
      if is_hand_part_of_made_up_hand(&hand, &htype, &assembled_hand) == false {
        continue;
      }
      if htype > flop_hand_type {
        *improved_hands_hash_map.entry(htype).or_insert(1) += 1;
      }
    }    
  }

  // now find what hands an opponent can potentially have based on community cards so far
  let mut remaining_deck = card_deck.to_vec();
  for i in 0..community_cards.len() {
    remaining_deck.retain(|&x| x != community_cards[i]);
  }
  remaining_deck.retain(|&x| x != hand[0]);
  remaining_deck.retain(|&x| x != hand[1]);

  let mut opponent_num_hands = 0;
  let mut total_eq = 0.0;
  let mut min_eq = 1000.0;
  let mut max_eq = 0.0;
  let mut opponent_hands_hash_map = HashMap::new();
  for i in 0..remaining_deck.len() {
    for j in (i+1)..remaining_deck.len() {
      let mut h = vec![remaining_deck[i],remaining_deck[j]];
      h.sort();
      // skip all really crappy hands that majority of players 'should' never play
      let (_, avg_eq, _) = starting_hands[&h];
      if avg_eq < 0.35 {
        continue; 
      }
      let (_, eq, htype, _) = get_best_hand(&h, &community_cards, &combinations);
      if eq < min_eq {
        min_eq = eq;
      }
      if eq > max_eq {
        max_eq = eq;
      }
      total_eq = total_eq+eq;
      opponent_num_hands = opponent_num_hands+1;
      *opponent_hands_hash_map.entry(htype).or_insert(1) += 1;
    }
  }

  // now calculate 'real' hand equity based on potential range of opponent hands
  let range_eq = max_eq-min_eq;
  let mut real_my_hand_eq = flop_equity-min_eq;
  if real_my_hand_eq < 0.0 {
    real_my_hand_eq = 0.0;
  }
  real_my_hand_eq = real_my_hand_eq/range_eq;

  return (flop_hand_type, real_my_hand_eq, improved_hands_hash_map, opponent_hands_hash_map, opponent_num_hands)
}