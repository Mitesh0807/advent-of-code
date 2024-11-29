/*
Advent of Code[About][Events][Shop][Settings][Log Out]Mitesh0807 2*
   $year=2021;[Calendar][AoC++][Sponsors][Leaderboard][Stats]
Our sponsors help make Advent of Code possible:
TwilioQuest - Learn to code and lead your intrepid crew on a mission to save The Cloud in TwilioQuest, a PC role-playing game inspired by classics of the 16-bit era. Free forever, and available now for Windows, Mac, and Linux.
--- Day 2: Dive! ---
Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

forward X increases the horizontal position by X units.
down X increases the depth by X units.
up X decreases the depth by X units.
Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2
Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

forward 5 adds 5 to your horizontal position, a total of 5.
down 5 adds 5 to your depth, resulting in a value of 5.
forward 8 adds 8 to your horizontal position, a total of 13.
up 3 decreases your depth by 3, resulting in a value of 2.
down 8 adds 8 to your depth, resulting in a value of 10.
forward 2 adds 2 to your horizontal position, a total of 15.
After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

To begin, get your puzzle input.

Answer:


You can also [Share] this puzzle.
*/

use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let calibrations = input.lines();
    let mut sum = 0;
    let mut previous_line: Option<i32> = None;

    for line in calibrations.into_iter() {
        let current_value: i32 = match line.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing line: {line}");
                continue;
            }
        };

        if let Some(previous) = previous_line {
            if previous < current_value {
                sum += 1;
            }
        }
        previous_line = Some(current_value);
    }

    println!("Sum: {sum}");
}
