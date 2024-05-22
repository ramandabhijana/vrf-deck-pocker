
use rand::rngs::OsRng;
use schnorrkel::{Keypair, MiniSecretKey, vrf::{VRFInOut, VRFProof, VRFSignature}};
use std::convert::TryInto;

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

fn extract_vrf_output(io: &VRFInOut) -> u64 {
    let output_bytes: [u8; 8] = io.make_bytes(&[]).as_ref()[..8].try_into().unwrap();
    u64::from_be_bytes(output_bytes)
}

fn generate_keypairs(count: usize) -> Vec<Keypair> {
    (0..count).map(|_| Keypair::generate_with(OsRng)).collect()
}

fn sign_card(card: u8, keypair: &Keypair) -> Signature {
    let ctx = signing_context(b"signed bisou");
    keypair.sign(ctx.bytes(&[card]))
}

fn evaluate_vrf(card: u8, keypair: &Keypair) -> (Vec<u8>, Signature) {
    let ctx = signing_context(b"example context");
    let output = vec![card];
    let proof = keypair.sign(ctx.bytes(&output));
    (output, proof)
}

fn verify_vrf(output: &Vec<u8>, proof: &Signature, public_key: &PublicKey) -> bool {
    let ctx = signing_context(b"example context");
    public_key.verify(ctx.bytes(output), proof).is_ok()
}


fn main() {

    let key_pairs_players = generate_keypairs(4);

    let mut distributed_cards = Vec::new();

    let (inout, proof) = generate_vrf_output(card, &keypairs[i]);
    for i in 0..=(52/4) {


        let signature = sign_card(deck[i], &keypairs[i]);
        distributed_cards.push(("p1", draw_some_cards(i), proof, keypairs[i].public));
        distributed_cards.push(("p2", draw_some_cards(i+1), proof, keypairs[i+1].public));
        distributed_cards.push(("p3", draw_some_cards(i+2), proof, keypairs[i+2].public));
        distributed_cards.push(("p4", draw_some_cards(i+3), proof, keypairs[i+3].public));

    }

    for (i, (card, output, proof, public_key)) in distributed_cards.iter().enumerate() {
        let is_valid = verify_vrf(output, proof, public_key);
        println!("Player {} has this card {}: verification {}", i + 1, card, if is_valid { "yes" } else { "sorry" });
    }



}