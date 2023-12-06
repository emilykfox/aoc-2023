use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct HandBid {
    hand: String,
    bid: u32,
}

fn main() {
    let lines = aoc_2023::collect_lines("./inputs/day07.txt").unwrap();

    let mut hand_bids = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        hand_bids.push(HandBid {
            hand: parts[0].to_string(),
            bid: parts[1].parse().unwrap(),
        })
    }
    hand_bids.sort_by(|first, second| {
        let mut first_map = HashMap::new();
        for byte in first.hand.bytes().by_ref() {
            if !first_map.contains_key(&byte) {
                first_map.insert(byte, 1);
            } else {
                let num = first_map.get_mut(&byte).unwrap();
                *num += 1;
            }
        }
        let mut second_map = HashMap::new();
        for byte in second.hand.bytes().by_ref() {
            if !second_map.contains_key(&byte) {
                second_map.insert(byte, 1);
            } else {
                let num = second_map.get_mut(&byte).unwrap();
                *num += 1;
            }
        }

        let first_max = *first_map.values().max().unwrap();
        let second_max = *second_map.values().max().unwrap();
        if first_max > second_max {
            std::cmp::Ordering::Greater
        } else if first_max < second_max {
            std::cmp::Ordering::Less
        } else if first_max == 3
            && first_map.values().find(|num| **num == 2).is_some()
            && !second_map.values().find(|num| **num == 2).is_some()
        {
            std::cmp::Ordering::Greater
        } else if first_max == 3
            && !first_map.values().find(|num| **num == 2).is_some()
            && second_map.values().find(|num| **num == 2).is_some()
        {
            std::cmp::Ordering::Less
        } else if first_max == 2
            && first_map.values().filter(|num| **num == 2).count() == 2
            && second_map.values().filter(|num| **num == 2).count() < 2
        {
            std::cmp::Ordering::Greater
        } else if first_max == 2
            && first_map.values().filter(|num| **num == 2).count() < 2
            && second_map.values().filter(|num| **num == 2).count() == 2
        {
            std::cmp::Ordering::Less
        } else {
            let mut return_val = std::cmp::Ordering::Greater;
            for (first_byte, second_byte) in first
                .hand
                .bytes()
                .by_ref()
                .zip(second.hand.bytes().by_ref())
            {
                let cards = vec![
                    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
                ];
                let first_pos = cards
                    .iter()
                    .position(|card| *card as u8 == first_byte)
                    .unwrap();
                let second_pos = cards
                    .iter()
                    .position(|card| *card as u8 == second_byte)
                    .unwrap();
                if first_pos < second_pos {
                    break;
                } else if second_pos < first_pos {
                    return_val = std::cmp::Ordering::Less;
                    break;
                }
            }
            return_val
        }
    });

    let mut total = 0;
    for (rankish, hand_bid) in hand_bids.iter().enumerate() {
        let rank = rankish + 1;
        if rank >= 200 && rank < 300 {
            println!("{}: {:?}", rank, hand_bid);
        }
        total += rank as u32 * hand_bid.bid;
    }

    println!("Part A: {}", total);

    hand_bids.sort_by(|first, second| {
        let mut first_map = HashMap::new();
        first_map.insert('J' as u8, 0);
        let mut j_count = 0;
        for byte in first.hand.bytes().by_ref() {
            if byte != 'J' as u8 {
                if !first_map.contains_key(&byte) {
                    first_map.insert(byte, 1);
                } else {
                    let num = first_map.get_mut(&byte).unwrap();
                    *num += 1;
                }
            } else {
                j_count += 1;
            }
        }
        let (max_card, _) = first_map.iter().max_by_key(|(_, num)| **num).unwrap();
        let max_card = *max_card;
        *first_map.get_mut(&max_card).unwrap() += j_count;

        j_count = 0;
        let mut second_map = HashMap::new();
        second_map.insert('J' as u8, 0);
        for byte in second.hand.bytes().by_ref() {
            if byte != 'J' as u8 {
                if !second_map.contains_key(&byte) {
                    second_map.insert(byte, 1);
                } else {
                    let num = second_map.get_mut(&byte).unwrap();
                    *num += 1;
                }
            } else {
                j_count += 1;
            }
        }
        let (max_card, _) = second_map.iter().max_by_key(|(_, num)| **num).unwrap();
        let max_card = *max_card;
        *second_map.get_mut(&max_card).unwrap() += j_count;

        let first_max = *first_map.values().max().unwrap();
        let second_max = *second_map.values().max().unwrap();
        if first_max > second_max {
            std::cmp::Ordering::Greater
        } else if first_max < second_max {
            std::cmp::Ordering::Less
        } else if first_max == 3
            && first_map.values().find(|num| **num == 2).is_some()
            && !second_map.values().find(|num| **num == 2).is_some()
        {
            std::cmp::Ordering::Greater
        } else if first_max == 3
            && !first_map.values().find(|num| **num == 2).is_some()
            && second_map.values().find(|num| **num == 2).is_some()
        {
            std::cmp::Ordering::Less
        } else if first_max == 2
            && first_map.values().filter(|num| **num == 2).count() == 2
            && second_map.values().filter(|num| **num == 2).count() < 2
        {
            std::cmp::Ordering::Greater
        } else if first_max == 2
            && first_map.values().filter(|num| **num == 2).count() < 2
            && second_map.values().filter(|num| **num == 2).count() == 2
        {
            std::cmp::Ordering::Less
        } else {
            let mut return_val = std::cmp::Ordering::Greater;
            for (first_byte, second_byte) in first
                .hand
                .bytes()
                .by_ref()
                .zip(second.hand.bytes().by_ref())
            {
                let cards = vec![
                    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
                ];
                let first_pos = cards
                    .iter()
                    .position(|card| *card as u8 == first_byte)
                    .unwrap();
                let second_pos = cards
                    .iter()
                    .position(|card| *card as u8 == second_byte)
                    .unwrap();
                if first_pos < second_pos {
                    break;
                } else if second_pos < first_pos {
                    return_val = std::cmp::Ordering::Less;
                    break;
                }
            }
            return_val
        }
    });

    let mut total = 0;
    for (rankish, hand_bid) in hand_bids.iter().enumerate() {
        let rank = rankish + 1;
        if rank >= 200 && rank < 300 {
            println!("{}: {:?}", rank, hand_bid);
        }
        total += rank as u32 * hand_bid.bid;
    }
    println!("Part B: {}", total);
}
