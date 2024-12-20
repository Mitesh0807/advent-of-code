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
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid command: {}", line);
            continue;
        }

        let command = parts[0];
        let value: i32 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid value in command: {}", line);
                continue;
            }
        };

        match command {
            "forward" => horizontal += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => println!("Unknown command: {}", command),
        }
    }

    println!("Horizontal: {}, Depth: {}", horizontal, depth);
    println!("Result (horizontal * depth): {}", horizontal * depth);

    Ok(())
}
