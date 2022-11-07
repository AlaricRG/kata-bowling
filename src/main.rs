fn bowling_score(score_sequence:Vec<u8>) -> u16 {
   score_sequence.into_iter().reduce(|a, b| a + b).unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_0_when_sequence_is_full_of_0() {
        let sequence = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert_eq!(bowling_score(sequence), 0);
    }
}