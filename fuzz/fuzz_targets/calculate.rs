#![no_main]
use game_of_life_in_rust::calculate;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (char, i64, i64)| {
    let _ = calculate(data.0, data.1, data.2);
});
