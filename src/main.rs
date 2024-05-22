use rand::seq::SliceRandom;
use rand::thread_rng;

fn shuffled_deck() -> impl Iterator<Item = u8> {
    // Create a deck of cards represented as numbers from 1 to 52
    let mut deck: Vec<u8> = (1..=52).collect();

    // Shuffle the deck
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    // Return an iterator over the shuffled deck
    deck.into_iter()
}

fn draw_some_cards(n: u8) -> Vec<u8>
{
    let deck_iterator = shuffled_deck();
    let vec: Vec<_> = deck_iterator.take(n as usize).collect();
    return vec
}





fn main() {
    let mut deck_iterator = shuffled_deck();

    // Draw cards from the iterator
    while let Some(card) = deck_iterator.next() {
        println!("Drew card: {}", card);
    }

    println!("No more cards in the deck!");
}
