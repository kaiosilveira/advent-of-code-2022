# Tuning Trouble

The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

## Part I: Identifying the start of the transmission

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

`mjqjpqmgbljsphdztnvjfqwrcgsmlb`

After the first three characters (`mjq`) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters `mjqj`. Because `j` is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are `jpqm`, which are all different. In this case, your subroutine should report the value `7`, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

- `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character `5`
- `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character `6`
- `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character `10`
- `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character `11`

**How many characters need to be processed before the first start-of-packet marker is detected?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

```rust
pub fn find_first_occurrence_of_all_different_chars_in(
    input: &str
) -> usize {
    let mut current_char = 1;
    let mut last_four_characters_processed = String::from("");

    for character in input.chars() {
        if last_four_characters_processed.len() < 4 {
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
```

</details>

## Part II: Identifying the start of the message

Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.
A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

- `mjqjpqmgbljsphdztnvjfqwrcgsmlb`: first marker after character `19`
- `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character `23`
- `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character `23`
- `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character `29`
- `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character `26`

**How many characters need to be processed before the first start-of-message marker is detected?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

```rust
pub fn find_first_occurrence_of_all_different_chars_in(
    input: &str,
    sequence_length: usize,
) -> usize {
    // -- snip --
```

```rust
if last_four_characters_processed.len() < sequence_length {
// -- snip
```

</details>
