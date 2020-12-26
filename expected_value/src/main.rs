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


/*
High card → 0 to 14
Pair → 15 to 29
Two Pair → 30 to 44
3 of a Kind → 45 to 64
Straight → 65 to 74
Flush → 75 to 89
Full House → 90 to 104
Four of a Kind → 105 to 119
Straight Flush →120 to 134
Royal Flush →134
*/
#[allow(illegal_floating_point_literal_pattern)]
fn get_best_hand_string(f: f32) -> String {
  let res = match f {
    /*0.0..=14.999 => "0. High card",
    15.0..=29.999 => "1. Pair",
    30.0..=44.999 => "2. Two Pairs",
    45.0..=64.999 => "3. Three of a Kind",
    65.0..=79.999 => "4. Straight",
    80.0..=89.999 => "5. Flush",
    90.0..=104.999 => "6. Full House",
    105.0..=119.999 => "7. Four of a Kind",
    120.0..=133.999 => "8. Straight Flush",
    134.0..=135.0 => "9. Royal Flush",*/
    0.0..=99.999 => "0. High card",
    100.0..=199.999 => "1. Pair",
    200.0..=299.999 => "2. Two Pairs",
    300.0..=399.999 => "3. Three of a Kind",
    400.0..=499.999 => "4. Straight",
    500.0..=599.999 => "5. Flush",
    600.0..=699.999 => "6. Full House",
    700.0..=799.999 => "7. Four of a Kind",
    800.0..=899.999 => "8. Straight Flush",
    900.0..=999.999 => "9. Royal Flush",
    _ => panic!("unknown range for {}", f),
  };
  return res.to_string()
}

// check if this hand contains hand cards or if it is made up from community cards only
// logic is that if there is a pair in community cards, that doesn't help me at all
// and so such "pair" should be skipped
fn should_skip_hand(hand_type: &str, h: &Vec<Card>, hand: &Vec<Card>) -> bool {
  if hand_type == "1. Pair" {
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
  if hand_type == "2. Two Pairs" {
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
  if hand_type == "3. Three of a Kind" {
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
fn get_best_hand(my_hand: &Vec<Card>, community: &Vec<Card>, combinations: &HashMap<Vec<Card>,(f32,f32)>) -> (f32, f32, String) {
  let mut sorted_cards = Vec::<Card>::new();
  for h in my_hand {
    sorted_cards.push(*h);
  }
  for h in community {
    sorted_cards.push(*h);
  }
  sorted_cards.sort();
  let (highest_value, equity) = match sorted_cards.len() {
    5 =>  {
      let highest_score;
      let highest_eq;
      let (score, eq) = combinations[&sorted_cards];
      let hand_type = get_best_hand_string(score);
      if should_skip_hand(&hand_type, &sorted_cards, &my_hand) && (hand_type == "1. Pair" || hand_type == "3. Three of a Kind") {
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
      }
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
        let hand_type = get_best_hand_string(score);
        if should_skip_hand(&hand_type, &new_hand, &my_hand) {
          continue;
        }
        if score > highest_score {
          highest_score = score;
          highest_eq = eq;
        }
      }
      (highest_score, highest_eq)
    },
    _ => panic!("unexpected cards len {} in get_best_hand", sorted_cards.len()),
  };
  return (highest_value, equity, get_best_hand_string(highest_value))
}

fn main() -> Result<(), Error> {
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
    let card_deck = conv_string_to_cards("2c 3c 4c 5c 6c 7c 8c 9c Tc Jc Qc Kc Ac 2h 3h 4h 5h 6h 7h 8h 9h Th Jh Qh Kh Ah 2s 3s 4s 5s 6s 7s 8s 9s Ts Js Qs Ks As 2d 3d 4d 5d 6d 7d 8d 9d Td Jd Qd Kd Ad");
    for i in 0..card_deck.len() {
      for j in (i+1)..card_deck.len() {
        let mut hand = vec![card_deck[i], card_deck[j]];
        hand.sort();
        let found_hands = find_possible_hands_in_all_combinations_no_ref(&hand, &combinations);
        let mut total_eq = 0.0;
        let num_hands = found_hands.len() as f32;
        for h in found_hands {
          let (hscore, hequity) = combinations[&h];
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

  let args: Vec<String> = env::args().collect();
  //println!("args: {:?}", args);
  let mut input: String = "C8 H5 H7 D12 D6".to_string();
  if args.len() == 2 {
    input = args[1].to_string();
  }

  let mut input_cards = conv_string_to_cards(&input);

  if input_cards.len() == 2 {
    input_cards.sort();
    let start_eq = starting_hands[&input_cards];
    println!("hand cards: {:?}, eq: {:.2}%", input_cards, start_eq*100.0);
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
  for i in 2..input_cards.len() {
    community.push(input_cards[i]);
  }

  println!("hand cards: {:?}", hand);
  println!("community cards: {:?}", community);

  let mut all_cards = Vec::<Card>::new();
  all_cards.extend(hand.to_vec().iter());
  all_cards.extend(community.to_vec().iter());

  let (flop_hand_score, flop_equity, flop_hand_type) = get_best_hand(&hand, &community, &combinations);

  let mut min_future_eq = 10000.0;
  let mut max_future_eq = 0.0;
  /*if all_cards.len() < 7 {
    let mut possible_hands = Vec::<Vec<&Card>>::new();

    if all_cards.len() == 5 {
      let c4 = all_cards.iter().combinations(4);
      let c3 = all_cards.iter().combinations(3);
      for c in c4 {
        possible_hands.push(c);
      }
      for c in c3 {
        possible_hands.push(c);
      }
    } else {
      let c4 = all_cards.iter().combinations(4);
      for c in c4 {
        possible_hands.push(c);
      }
    }
    let mut num_of_better_hands = HashMap::new();
    let mut num_possible_hands = 0;
    let mut seen_hands = HashSet::new();
    for c in possible_hands {
      let found_hands = find_possible_hands_in_all_combinations(&c, &combinations);
      for h in found_hands {
        if seen_hands.contains(&h) {
          //println!("found already seen hand: {:?}", h);
          continue;
        } else {
          seen_hands.insert(h.to_vec());
        }
        num_possible_hands = num_possible_hands+1;
        let (hscore, hequity) = combinations[&h];
        if hscore > flop_hand_score {
          let hand_type = get_best_hand_string(hscore).clone();
          // i need to exclude hands that improve community cards. as that doesn't help me at all. 
          // so lets filter out hands that do not contain my hand cards at all.
          if find_common_cards_in_pack_no_ref(&hand, &h) == 0 {
            continue;
          }
          if hequity > max_future_eq {
            max_future_eq = hequity;
          }
          if hequity < min_future_eq {
            min_future_eq = hequity;
          }

          if should_skip_hand(&hand_type, &h, &hand) {
            continue;
          }
          if hand_type == "0. High card" {
            // those will never improve my original hand, so we can skip all of them safely
            continue;
          }

          if !num_of_better_hands.contains_key(&hand_type) {
            num_of_better_hands.insert(hand_type.clone(), 1);
          } else {
            let next_num = num_of_better_hands[&hand_type]+1;
            num_of_better_hands.remove(&hand_type);
            num_of_better_hands.insert(hand_type.clone(), next_num);
          }

          //println!("Better possible hand: {:?} - {}", h, hand_type);  
        }
      }
    }
    //println!("Potential future equity if hand improves: min: {}, max: {}, mean: {}", min_future_eq, max_future_eq, (min_future_eq+max_future_eq)*0.5);
    //println!("num hands: {}", num_possible_hands);
    let mut sorted_keys: Vec<&String> = num_of_better_hands.keys().collect();
    sorted_keys.sort();
    for hand_type in sorted_keys {
      println!("Num of better hands: {}, {}, {}%", hand_type, num_of_better_hands[hand_type], (num_of_better_hands[hand_type] as f32/num_possible_hands as f32)*100.0);
    }
  }
  let mut future_eq = flop_equity;
  if max_future_eq != 0.0 {
    future_eq = (min_future_eq+max_future_eq)*0.5;
  }*/
  println!("hand score: {}, equity: {:.2}%, type: {}", flop_hand_score, flop_equity*100.0, flop_hand_type);

  // now find what hands an opponent can potentially have based on community cards so far
  /*let mut community_cards = Vec::<Card>::new();
  community_cards.extend(community.to_vec().iter());

  println!("community cards: {:?}", community_cards);

  let comm_cards_c3 = community_cards.iter().combinations(3);
  let comm_cards_c4 = community_cards.iter().combinations(4);

  let mut possible_hands = Vec::<Vec<&Card>>::new();
  for c in comm_cards_c3 {
    possible_hands.push(c);
  }
  for c in comm_cards_c4 {
    possible_hands.push(c);
  }

  let mut num_of_better_hands = HashMap::new();
  let mut num_possible_hands = 0;
  let mut seen_hands = HashSet::new();
  for c in possible_hands {
    let found_hands = find_possible_hands_in_all_combinations(&c, &combinations);
    for h in found_hands {
      if seen_hands.contains(&h) {
        continue;
      } else {
        seen_hands.insert(h.to_vec());
      }
      num_possible_hands = num_possible_hands+1;
      let (hscore, _hequity) = combinations[&h];
      let hand_type = get_best_hand_string(hscore).clone();
      if !num_of_better_hands.contains_key(&hand_type) {
        num_of_better_hands.insert(hand_type.clone(), 1);
      } else {
        let next_num = num_of_better_hands[&hand_type]+1;
        num_of_better_hands.remove(&hand_type);
        num_of_better_hands.insert(hand_type.clone(), next_num);
      }
      //println!("possible hand: {:?} - {}", h, hand_type);  
    }
  }
  let mut sorted_keys: Vec<&String> = num_of_better_hands.keys().collect();
  sorted_keys.sort();
  for hand_type in sorted_keys {
    println!("Num of better hands: {}, {}, {}%", hand_type, num_of_better_hands[hand_type], (num_of_better_hands[hand_type] as f32/num_possible_hands as f32)*100.0);
  }*/ 
 
  Ok(())
}