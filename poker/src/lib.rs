/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut result = vec![];

    let face_values = HashMap::from([("J", 11), ("Q", 12), ("K", 13), ("A", 14)]);

    let mut highest = 0;

    for hand in hands {
        let hand_value = 
    }

    if hands.len() == 1 {
        result.push(hands[0]);
    }
    result
}
