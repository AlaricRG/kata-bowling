fn get_bowling_score(score_sequence:Vec<u8>) -> u16 {
    let spare_indices = get_indices_of_scores_to_double_because_of_spare(&score_sequence);
    score_sequence
        .into_iter()
        .enumerate()
        .map(|(index, score)| {
            if spare_indices.contains(&index) {
                score * 2
            } else {
                score
            }
        })
        .reduce(|a, b| a + b).unwrap().into()
}

fn get_indices_of_scores_to_double_because_of_spare(score_sequence: &Vec<u8>) -> Vec<usize>{
    let mut indices_to_return = Vec::new();
    let spare_score = 10;
    for (i, knocked_pins) in score_sequence.chunks_exact(2).enumerate() {
        if knocked_pins[0] + knocked_pins[1] == spare_score {
            indices_to_return.push(i*2+2)
        }
    }
    indices_to_return
}

#[cfg(test)]
mod tests_basics {
    use super::*;

    #[test]
    fn should_return_0_when_sequence_is_full_of_0() {
        let sequence = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert_eq!(get_bowling_score(sequence), 0);
    }

    #[test]
    fn should_return_1_when_sequence_has_1_then_full_0() {
        let sequence = vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert_eq!(get_bowling_score(sequence), 1);
    }

    mod test_spare {
        use super::*;

        #[test]
        fn should_calcul_score_when_1_spare() {
            let sequence = vec![2,8,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
            assert_eq!(get_bowling_score(sequence), 16);
        }

        #[test]
        fn should_calculate_score_when_2_spares() {
            let sequence = vec![2,8,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,5,4];
            assert_eq!(get_bowling_score(sequence), 16 + 18);
        }
    }
}

fn main() {}