pub fn find_first_occurrence_of_all_different_chars_in(
    input: &str,
    sequence_length: usize,
) -> usize {
    let mut current_char = 1;
    let mut last_four_characters_processed = String::from("");

    for character in input.chars() {
        if last_four_characters_processed.len() < sequence_length {
            last_four_characters_processed.push(character);
        } else {
            last_four_characters_processed.remove(0);
            last_four_characters_processed.push(character);

            let mut char_pos = 0;
            let mut all_different = true;
            for char in last_four_characters_processed.chars() {
                let slice = &last_four_characters_processed[char_pos + 1..];

                if slice.contains(char) {
                    all_different = false;
                    break;
                }

                char_pos += 1;
            }

            if all_different {
                break;
            }
        }

        current_char += 1;
    }

    current_char
}

pub fn find_start_of_packet(buffer: &str) -> usize {
    let sequence_length = 4;
    find_first_occurrence_of_all_different_chars_in(buffer, sequence_length)
}

pub fn find_start_of_message(buffer: &str) -> usize {
    let sequence_length = 14;
    find_first_occurrence_of_all_different_chars_in(buffer, sequence_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        assert_eq!(7, find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(11, find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
        assert_eq!(
            10,
            find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
    }

    #[test]
    fn test_start_of_message() {
        assert_eq!(19, find_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, find_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, find_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(
            29,
            find_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            26,
            find_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
}
