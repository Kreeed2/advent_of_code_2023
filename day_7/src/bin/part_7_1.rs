use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
enum Cards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Cards {
    pub fn get_rank(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug)]
enum Hands {
    HighCard,
    Pair,
    TwoPairs,
    Trips,
    FullHouse,
    Quads,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Cards>,
}

impl Hand {
    fn new(cards: &str) -> Self {
        Self {
            cards: Self::parse_cards(cards),
        }
    }

    fn parse_cards(cards: &str) -> Vec<Cards> {
        cards
            .chars()
            .map(|card| match card {
                '2' => Cards::Two,
                '3' => Cards::Three,
                '4' => Cards::Four,
                '5' => Cards::Five,
                '6' => Cards::Six,
                '7' => Cards::Seven,
                '8' => Cards::Eight,
                '9' => Cards::Nine,
                'T' => Cards::Ten,
                'J' => Cards::Jack,
                'K' => Cards::King,
                'Q' => Cards::Queen,
                'A' => Cards::Ace,
                _ => panic!("Invalid card"),
            })
            .collect()
    }
    
    fn calculate_first_rank(&self) -> i32 {
        let mut ranks: Vec<usize> = vec![0; 13];
        let mut hands: Vec<bool> = vec![false; 7];
        let mut found_hand = false;
        for card in self.cards.iter() {
            ranks[card.get_rank()] += 1;
        }

        let mut pair_one: i32 = -1;
    	let mut pair_two: i32 = -1;
    	let mut trips: i32 = -1;

    	for (i, rank) in (0..).zip(ranks.iter()) {
    		if *rank > 1 {
                if *rank == 5 {
                    hands[Hands::FiveOfAKind as usize] = true;
                    found_hand = true;
                    break;
                } else if *rank == 4 {
    				hands[Hands::Quads as usize] = true;
    				found_hand = true;
    				break;
    			} else if *rank == 3 {		    
    				trips = i;
    			} else {
    				if pair_one == -1 {
    					pair_one = i;
    				} else {
    					pair_two = pair_one;
    					pair_one = i;
    				}
    			}
    		}
    	}
    	if trips != -1 && pair_one != -1 {
    		hands[Hands::FullHouse as usize] = true;
    		found_hand = true;
    	} else if trips != -1 && pair_one == -1 {
    		hands[Hands::Trips as usize] = true;
    		found_hand = true;
    	} else if pair_one != -1 && pair_two != -1 {
    		hands[Hands::TwoPairs as usize] = true;
    		found_hand = true;
    	} else if pair_one != -1 {
    		hands[Hands::Pair as usize] = true;
    		found_hand = true;
    	}

        if !found_hand {
    		// High card
    		hands[Hands::HighCard as usize] = true;
    	}

        // Calculate rank
		let mut rank = -1;
		if hands[Hands::FiveOfAKind as usize] { // FIVE OF A KIND
		    rank = Hands::FiveOfAKind as i32;
		} else if hands[Hands::Quads as usize] { // FOUR OF A KIND
		    rank = Hands::Quads as i32;
		} else if hands[Hands::FullHouse as usize] { // FULL HOUSE
		    rank = Hands::FullHouse as i32;
		} else if hands[Hands::Trips as usize] { // TRIPS
		    rank = Hands::Trips as i32;
		} else if hands[Hands::TwoPairs as usize] { // TWO PAIRS
		    rank = Hands::TwoPairs as i32;
		} else if hands[Hands::Pair as usize] { // ONE PAIR
		    rank = Hands::Pair as i32;
		} else if hands[Hands::HighCard as usize] { // HIGH CARD
		    rank = Hands::HighCard as i32;
        }
		rank
    }

    fn calculate_second_rank(&self, other: &Self) -> Option<Ordering> {
        for (index, card) in self.cards.iter().enumerate() {
            match &card.partial_cmp(&other.cards[index]) {
                Some(Ordering::Equal) => continue,
                ordering => return *ordering,
            }            
        }
        Some(Ordering::Equal)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_rank = self.calculate_first_rank();
        let other_rank = other.calculate_first_rank();
        if self_rank > other_rank {
            return Some(Ordering::Greater);
        } else if self_rank < other_rank {
            return Some(Ordering::Less);
        } else {
            return self.calculate_second_rank(other);
        }
    }
    
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Hand {
    
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
    
}

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_1(input);
    dbg!(output);
}


fn part_1(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        hands.push((Hand::new(parts[0]), parts[1].parse().unwrap()));
    }

    hands.sort_by_key(|key| key.0.clone());

    hands.into_iter().enumerate().map(|(pos, (_, bit))| bit * (pos as u32 + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_cards() {
        assert_eq!(
            Hand::parse_cards("2TJQKA"),
            vec![
                Cards::Two,
                Cards::Ten,
                Cards::Jack,
                Cards::Queen,
                Cards::King,
                Cards::Ace,
            ]
        );
        assert_eq!(
            Hand::parse_cards("3456789"),
            vec![
                Cards::Three,
                Cards::Four,
                Cards::Five,
                Cards::Six,
                Cards::Seven,
                Cards::Eight,
                Cards::Nine,
            ]
        );
        // Add more test cases here
    }

    #[test]
    fn test_calculate_rank() {
        
        assert_eq!(Hand::new("32T3K").calculate_first_rank(), 
                Hands::Pair as i32);
        assert_eq!(Hand::new("T55J5").calculate_first_rank(), 
                Hands::Trips as i32);
        // Add more test cases here
    }

    #[test]
    fn test_calculate_rank_2() {
        assert_eq!(Hand::new("KK677").calculate_second_rank(&Hand::new("KTJJT")), 
                Some(Ordering::Greater));
        // Add more test cases here
    }

    #[test]
    fn test_partial_ord() {
        let mut input = vec!["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].iter().map(|s| Hand::new(s)).collect::<Vec<_>>();
        let output = vec!["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"].iter().map(|s| Hand::new(s)).collect::<Vec<_>>();

        input.sort();
        assert_eq!(input, output);
    }

    #[test]
    fn test_part_1() {
        let input = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(part_1(input), 6440);
    }
}
