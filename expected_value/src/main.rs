#![allow(unused_imports)]
#![allow(dead_code)]

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
use std::time::{Duration, Instant};
use std::cmp;


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
  rank: u8,
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

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
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

fn should_skip_hand_with_pair(hand_type: &HandRank, h: &Vec<Card>, hand: &Vec<Card>) -> bool {
  if *hand_type == HandRank::Pair {
    // pair should be formed using one of my hand cards
    // so let's find that pair and compare rank to my hand cards
    let mut should_skip = false;
    for i in 0..h.len() {
      for j in (i+1)..h.len() {
        if h[i].rank == h[j].rank {
          if h[i].rank != hand[0].rank && h[i].rank != hand[1].rank {
            should_skip = true;
          }
          break;
        }
      }
    }
    if should_skip {
      //println!("skipping pair: {:?}", h);
      return true;
    }
  }
  return false
}
// check if this hand contains hand cards or if it is made up from community cards only
// logic is that if there is a pair in community cards, that doesn't help me at all
// and so such "pair" should be skipped
fn should_skip_hand(hand_type: &str, h: &Vec<Card>, hand: &Vec<Card>) -> bool {
  if hand_type == "1/9 Pair" {
    // pair should be formed using one of my hand cards
    // so let's find that pair and compare rank to my hand cards
    let mut should_skip = false;
    for i in 0..h.len() {
      for j in (i+1)..h.len() {
        if h[i].rank == h[j].rank {
          if h[i].rank != hand[0].rank && h[i].rank != hand[1].rank {
            should_skip = true;
          }
          break;
        }
      }
    }
    if should_skip {
      //println!("skipping pair: {:?}", h);
      return true;
    }
  }
  if hand_type == "2/9 Two Pairs" {
    // find both pairs
    let mut should_skip = false;
    // find first pair rank
    let mut first_pair_rank = 0;
    for i in 0..h.len() {
      for j in (i+1)..h.len() {
        if h[i].rank == h[j].rank {
          first_pair_rank = h[i].rank;
          break;
        }
      }
    }
    // find second pair now
    for i in 0..h.len() {
      for j in (i+1)..h.len() {
        if h[i].rank == h[j].rank && h[i].rank != first_pair_rank {
          if !((h[i].rank == hand[0].rank || h[i].rank == hand[1].rank) && (first_pair_rank == hand[0].rank || first_pair_rank == hand[1].rank)) {
            should_skip = true;
          }
          break;
        }
      }
    }          
    if should_skip {
      //println!("skipping two pair: {:?}", h);
      return true;
    }
  }
  if hand_type == "3/9 Three of a Kind" {
    let mut should_skip = false;
    for i in 0..h.len() {
      for j in (i+1)..h.len() {
        for k in (j+1)..h.len() {
          if h[i].rank == h[j].rank && h[i].rank == h[k].rank {
            if h[i].rank != hand[0].rank && h[i].rank != hand[1].rank {
              should_skip = true;
            }
            break;
          }
        }
      }
    }
    if should_skip {
      //println!("skipping 3 of a kind: {:?}", h);
      return true;
    }
  }  

  return false
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
      /*let hand_type = get_best_hand_string(score);
      if should_skip_hand(&hand_type, &sorted_cards, &my_hand) && (hand_type == "1/9 Pair" || hand_type == "3/9 Three of a Kind") {
        // there are several special cases which we want to disregard on the flop
        // if there is a pair in the flop, or if there is 3 of a kind in the flop
        // that's the only two combinations that can happen that will not include
        // any of my hand cards
        // so if this happens, artificially downgrade the hand
        highest_score = 7.0;
        highest_eq = 0.10;
      } else {
        highest_score = score;
        highest_eq = eq;
      }*/
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
        //let hand_type = get_best_hand_string(score);
        //if should_skip_hand(&hand_type, &new_hand, &my_hand) {
        //  continue;
        //}
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

fn main() -> Result<(), Error> {
  //let start_init_ts = Instant::now();

  let card_deck = conv_string_to_cards("2c 3c 4c 5c 6c 7c 8c 9c Tc Jc Qc Kc Ac 2h 3h 4h 5h 6h 7h 8h 9h Th Jh Qh Kh Ah 2s 3s 4s 5s 6s 7s 8s 9s Ts Js Qs Ks As 2d 3d 4d 5d 6d 7d 8d 9d Td Jd Qd Kd Ad");

  let mut combinations = HashMap::new();
  let mut starting_hands = HashMap::new();
  if Path::new("/home/pavel/nvme/GitHub/poker_ml/data.bin").exists() {
    let f = BufReader::new(File::open("/home/pavel/nvme/GitHub/poker_ml/data.bin").unwrap());
    combinations = bincode::deserialize_from(f).unwrap();
  } else {
    let vec = read(File::open("/home/pavel/nvme/GitHub/poker_ml/expected_value/hands.csv")?)?;
    for i in 0..vec.len() {
      let v = &vec[i];
      let equity = i as f32/vec.len() as f32;
      //println!("{}: {}, {}", i, v.value, equity);
      // key is hand set. value is (raw value of the card, hand equity as chance to win with that hand)
      combinations.insert(v.cards.to_vec(), (v.value, equity));
    }
  
    let mut f = BufWriter::new(File::create("~/nvme/GitHub/poker_ml/data.bin").unwrap());
    bincode::serialize_into(&mut f, &combinations).unwrap();
  }

  if Path::new("/home/pavel/nvme/GitHub/poker_ml/starting_hands.bin").exists() {
    let f = BufReader::new(File::open("/home/pavel/nvme/GitHub/poker_ml/starting_hands.bin").unwrap());
    starting_hands = bincode::deserialize_from(f).unwrap();
  } else {
    for i in 0..card_deck.len() {
      for j in (i+1)..card_deck.len() {
        let mut hand = vec![card_deck[i], card_deck[j]];
        hand.sort();
        let found_hands = find_possible_hands_in_all_combinations_no_ref(&hand, &combinations);
        let mut total_eq = 0.0;
        let num_hands = found_hands.len() as f32;
        for h in found_hands {
          let (_, hequity) = combinations[&h];
          total_eq = total_eq + hequity;
        }
        let aver_eq = total_eq/num_hands;
        println!("starting hand: {:?} - {:.2}%", hand, aver_eq*100.0);

        starting_hands.insert(hand, aver_eq);
      }
    }

    let mut f = BufWriter::new(File::create("/home/pavel/nvme/GitHub/poker_ml/starting_hands.bin").unwrap());
    bincode::serialize_into(&mut f, &starting_hands).unwrap();
  }

  //let duration_init = start_init_ts.elapsed();
  //println!("Init duration is: {:?}", duration_init);
  //let start_main_ts = Instant::now();

  /*for k in starting_hands.keys() {
    let v = starting_hands[k];
    if v < 0.48 {
      println!("{:?}", k);
    }
  }*/

  let args: Vec<String> = env::args().collect();
  //println!("args: {:?}", args);
  let mut input: String = "C8 H5 H7 D12 D6".to_string();
  let mut pot_str: String = "Total pot: $1.30\nMain pot: $1.10\n\n".to_string();
  if args.len() > 1 {
    input = args[1].to_string();
    pot_str = args[2].to_string();
  }

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

  if input_cards.len() == 2 {
    input_cards.sort();
    let start_eq = starting_hands[&input_cards];
    let action;
    if start_eq < 0.5 {
      action = "FOLD";
    } else {
      action = "CALL";
    }
    println!("hand cards: {:?}, Avg Eq: {:.2}%, Action: {}", input_cards, start_eq*100.0, action);
    return Ok(())
  }
  if input_cards.len() < 5 {
    println!("not enough cards, only got: {:?}", input_cards);
    return Ok(())
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
  println!("Pot: ${:.2}, To Call: ${:.2}", total_pot, call_amount);

  let mut all_cards = Vec::<Card>::new();
  all_cards.extend(hand.to_vec().iter());
  all_cards.extend(community.to_vec().iter());

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
      if should_skip_hand_with_pair(&htype, &assembled_hand, &hand) {
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
  let num_cards_in_deck_left = remaining_deck.len();

  let mut num_hands = 0;
  let mut total_eq = 0.0;
  let mut min_eq = 1000.0;
  let mut max_eq = 0.0;
  let mut opponent_hands_hash_map = HashMap::new();
  for i in 0..remaining_deck.len() {
    for j in (i+1)..remaining_deck.len() {
      let mut h = vec![remaining_deck[i],remaining_deck[j]];
      h.sort();
      // skip all really crappy hands that majority of players 'should' never play
      let starting_hand_eq = starting_hands[&h];
      if starting_hand_eq < 0.48 {
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
      num_hands = num_hands+1;
      *opponent_hands_hash_map.entry(htype).or_insert(1) += 1;
    }
  }
  //let oppon_eq = total_eq/num_hands as f32;

  // now calculate 'real' hand equity based on potential range of opponent hands
  let range_eq = max_eq-min_eq;
  let mut real_my_hand_eq = flop_equity-min_eq;
  if real_my_hand_eq < 0.0 {
    real_my_hand_eq = 0.0;
  }
  real_my_hand_eq = real_my_hand_eq/range_eq;
  // show my hands relative strength to any opponent's hand. essentially it is my equity
  println!("RelStr: {:.2}%, Type: {}", real_my_hand_eq*100.0, flop_hand_type);
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
    println!("{:<20}:{:.1}%", s, (opponent_hands_hash_map[&hand_type] as f32/num_hands as f32)*100.0);
  }

  //let duration_main = start_main_ts.elapsed();
  //println!("Main duration is: {:?}", duration_main);

  Ok(())
}