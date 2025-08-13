use crate::tutorial::common_util::{alpha_string_from_seed, print_header};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub fn bonus_leetcode_76_min_sub_string_contains_t_chars_including_dupes() {
    print!("==> Bonus Leetcode 76 Minimum Sub String of s that contains all t chars ");
    print!("including duplicates - sliding window with hashmaps counts of required and ");
    print!("current frequencies, and const target frequency comparison from initial ");
    println!("required frequency hashmap construction - O(n)");

    let columns = ["Data Size", "Time (µs)"];
    print_header(&columns);
    let start_size = 1024;
    let number_of_doubles = 8;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time: Vec<Duration> = Vec::with_capacity(1000);
        let s = alpha_string_from_seed(123456789, *size);
        let t = String::from(&s[(s.len() - 25)..(s.len() - 10)]);

        for _ in 0..1000 {
            let start = Instant::now();
            min_window(&s, &t);
            arr_time.push(start.elapsed());
        }

        let time = Duration::from_nanos(
            (arr_time.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time.len() as u128) as u64,
        );

        println!(
            "  {0:>1$} | {2:3$} ",
            size,
            columns[0].chars().count() - 1,
            time.as_micros(),
            columns[1].chars().count() - 1
        );
    }

    print!("\n\n");
}

// Given two strings s and t of lengths m and n respectively, return the minimum
// window substring of s such that every character in t (including duplicates)
// is included in the window. If there is no such substring, return the empty
// string "".
//
// I have no idea why they always have to have questions that sound like it comes
// out of a set theory math book, they basically asking find us the smallest,
// substring of s that include at least 1 of each of the characters of t.
//
// Well its going to be a sliding window problem for sure... since we wouldn't
// want to revisit stuff in s already processed.
//
// like wise we would want to reduce t to some sort of easily verifiable value on
// the slice of s under examination.
// -- algo idea: I think this will work--
// So I guess we need a hashmap of all characters in t first called thash and record
// the length of thash as thlen. Then we process s with a left/right boundry variable
// sliding window starting at l=0 r=0 with a hashmap called shash and building the
// state machine which works in steps of find qualifying window, contract left,
// slide smallest window right till finding the next qualifying window, and then
// looping back into the contract left step repeating and so on.
//
// Every time r+=1 occurs we increment a count of the occurances of the string's [r-1]
// position in the shash, every time we l+=1 we decrement the count of the occurances
// of the l-1 character from shash, if it falls to 0 we remove it from shash.
//
// In the 3 steps mentioned, when a comparison is needed if the window is a qualifying
// window or not check every element in thash if it has an element in shash. This part
// I don't really know how to make faster..., but I'm thinking on it.. faster aproaches
// must exist... Hmm maybe I can think of something faster, what if:
//      I keep a extra variable match_count whenever I add a new element in shash which
//      already exists in t I increment the match count.
//      Whenever the shash falls to a count of 0 and the element is removed we decrement
//      match_count, therefore if match_count==thlen then the current window qualifies.
// Yes this will work.
// The final result should be O(n+t)
//
//
// OK so I originally made a mistake: duplicate characters in t means that
// the result from s should have the same number of duplicates,
// this means that when thashmap is constructed the value portion of each
// required character key should be a count of how many is required.
//
// Then instead of comparing match_count = thash.len() to determine validity
// we compare match_count to thash.sum()
// this necesitates one additional change: instead of increasing match_count
// on the first itteration a new char gets added into shash we need to keep
// incrementing match_count while the count of the character in shash is <
// the count of the character in thash.
// Likewise instead of decrementing when shash's character value reaches 0
// we will now decrement while shash's value is moved < thash for said character
//
// NOTE THE ABOVE VARIABLE NAMES ETC IS NOT WHATS USED IN THE CODE.
enum SearchMode {
    ExpandRight,
    ContractLeft,
    SlideRight,
}
pub fn min_window(s: &String, t: &String) -> String {
    // nums is positive integers
    // target is posive integer
    // find length of shortest slice, that's sum is >= target
    // if it doesnt exist return 0
    if s.len() == 0 || t.len() == 0 {
        return String::from("");
    }

    let mut l_idx = 0;
    let mut r_idx: usize = 0;
    let mut search_mode = SearchMode::ExpandRight;
    let mut required_chars_required_freq_map = HashMap::with_capacity(t.len());
    let mut total_required_chars_needed = 0;
    t.chars().for_each(|c| {
        *required_chars_required_freq_map.entry(c).or_insert(0) += 1;
        total_required_chars_needed += 1;
    });
    let mut found_required_chars_freq_map = HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    let mut total_required_chars_found: usize = 0;
    // result = (left,right) char indexes
    let mut result_left_idx = 0;
    let mut result_right_idx = 0;
    loop {
        match search_mode {
            SearchMode::ExpandRight => {
                // Find the first slice that puts us at or over the target
                r_idx += 1;
                let r_char = &s_chars[r_idx - 1];
                // If r_char is a character with a required_char_frequency
                if let Some(required_char_frequency) = required_chars_required_freq_map.get(r_char)
                {
                    // Increment it's found frequency
                    let current_found_frequency =
                        found_required_chars_freq_map.entry(r_char).or_insert(0);
                    *current_found_frequency += 1;

                    // If it's found frequency was pushed to below or at the required frequency
                    if *current_found_frequency <= *required_char_frequency {
                        // Increment the total required characters we have found.
                        total_required_chars_found += 1;
                    }
                }

                // If we have reached the total_required_chars_needed for a qualifying
                // window
                if total_required_chars_found == total_required_chars_needed {
                    // Record the window
                    result_right_idx = r_idx;
                    result_left_idx = 0;
                    // Switch Mode to look for a shorter left.
                    search_mode = SearchMode::ContractLeft;
                } else {
                    // We still don't have a valid window.
                    // Check if we have reached the end of the r_idx range
                    if r_idx == s_chars.len() {
                        // We have reached the end with no more work todo.
                        break;
                    }
                }
            }
            SearchMode::ContractLeft => {
                l_idx += 1;
                let l_char_dropping = &s_chars[l_idx - 1];
                // If l_char_dropping is a character with a required_char_frequency
                if let Some(required_char_frequency) =
                    required_chars_required_freq_map.get(l_char_dropping)
                {
                    // If we can find it's current recorded found frequency
                    if let Some(current_found_frequency) =
                        found_required_chars_freq_map.get_mut(l_char_dropping)
                    {
                        // Decrement it's found frequency.
                        *current_found_frequency -= 1;

                        // If it's found frequency was pulled below the required_char_frequency
                        // then the removal of l_char has removed a required character.
                        if *current_found_frequency < *required_char_frequency {
                            // Decrement the total required characters we have found.
                            total_required_chars_found -= 1;
                        }

                        // If current_found_frequency has reached 0 remove it from the map
                        if *current_found_frequency == 0 {
                            found_required_chars_freq_map.remove(l_char_dropping);
                        }
                    }
                }

                // If we have reached the total_required_chars_needed for a qualifying
                // window
                if total_required_chars_found == total_required_chars_needed {
                    // Record the window
                    result_left_idx = l_idx;
                    result_right_idx = r_idx;
                } else {
                    // If we are here we are switching to looking for slide right
                    if r_idx == s_chars.len() {
                        // There is no right side left
                        // this means we are done.
                        break;
                    }
                    // If we no longer have a valid window, and there is
                    // characters left on the right, then we return to
                    // looking for a smaller window of the current bad
                    // window size on the right
                    search_mode = SearchMode::SlideRight;
                }
            }
            SearchMode::SlideRight => {
                // Slide the window right, removing the characters dropping off,
                // and adding the characters coming into the window on the
                // hashmaps updating total_required_characters_matched up and down
                // as needed.

                // Process the left character falling off.
                l_idx += 1;
                let l_char_dropping = &s_chars[l_idx - 1];
                // If l_char_dropping is a character with a required_char_frequency
                if let Some(required_char_frequency) =
                    required_chars_required_freq_map.get(l_char_dropping)
                {
                    // If we can find it's current recorded found frequency
                    if let Some(current_found_frequency) =
                        found_required_chars_freq_map.get_mut(l_char_dropping)
                    {
                        // Decrement it's found frequency.
                        *current_found_frequency -= 1;

                        // If it's found frequency was pulled below the required_char_frequency
                        // then the removal of l_char has removed a required character.
                        if *current_found_frequency < *required_char_frequency {
                            // Decrement the total required characters we have found.
                            total_required_chars_found -= 1;
                        }

                        // If current_found_frequency has reached 0 remove it from the map
                        if *current_found_frequency == 0 {
                            found_required_chars_freq_map.remove(l_char_dropping);
                        }
                    }
                }

                // Process the right character getting added.
                r_idx += 1;
                let r_char = &s_chars[r_idx - 1];
                // If r_char is a character with a required_char_frequency
                if let Some(required_char_frequency) = required_chars_required_freq_map.get(r_char)
                {
                    // Increment it's found frequency
                    let current_found_frequency =
                        found_required_chars_freq_map.entry(r_char).or_insert(0);
                    *current_found_frequency += 1;

                    // If it's found frequency was pushed to below or at the required frequency
                    if *current_found_frequency <= *required_char_frequency {
                        // Increment the total required characters we have found.
                        total_required_chars_found += 1;
                    }
                }

                // If we have reached the total_required_chars_needed for a qualifying
                // window
                if total_required_chars_found == total_required_chars_needed {
                    // Record the window
                    result_left_idx = l_idx;
                    result_right_idx = r_idx;
                    // Switch Mode to look for a shorter left.
                    search_mode = SearchMode::ContractLeft;
                } else {
                    // We still don't have a valid window.
                    // Check if we have reached the end of the r_idx range
                    if r_idx == s_chars.len() {
                        // We have reached the end with no more work todo.
                        break;
                    }
                }
            }
        }
    }
    if result_right_idx == 0 {
        // No result was found
        String::from("")
    } else {
        let result = &s_chars[result_left_idx..result_right_idx];
        let mut result_string = String::with_capacity(result.len());
        for &c in result {
            result_string.push(c);
        }
        result_string
    }
}
