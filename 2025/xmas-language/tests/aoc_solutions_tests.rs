/// Regression tests for Advent of Code solutions.
///
/// These tests ensure that existing solutions continue to work
/// after language changes and updates. Each test verifies that
/// a solution produces the expected answer.

use std::fs;
use xmas_language::{Lexer, Parser, Interpreter, Value};

/// Run a xmas program file with input and return the result
fn run_xmas_solution(program_path: &str, input_path: &str) -> Result<Value, String> {
    let code = fs::read_to_string(program_path)
        .map_err(|e| format!("Failed to read program file {}: {}", program_path, e))?;

    let input = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read input file {}: {}", input_path, e))?;

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens, code);
    let program = parser.parse()?;
    let mut interpreter = Interpreter::new();
    interpreter.set_input(&input);
    interpreter.interpret(&program)
}

/// Test: Advent of Code 2025, Day 1, Part 1
///
/// Challenge: Count how many times the dial points at 0 after any rotation.
/// Expected answer: 1195
#[test]
fn test_aoc_2025_day01_part1() {
    let result = run_xmas_solution(
        "../aoc-2025/day-01/part1.xmas",
        "../aoc-2025/day-01/input.txt"
    ).unwrap();

    assert_eq!(result, Value::Number(1195), "Day 1 Part 1 should return 1195");
}

/// Test: Advent of Code 2025, Day 1, Part 2
///
/// Challenge: Count how many times the dial points at 0 during rotations
/// (not just at the end), using password method 0x434C49434B.
/// Expected answer: 6770
#[test]
fn test_aoc_2025_day01_part2() {
    let result = run_xmas_solution(
        "../aoc-2025/day-01/part2.xmas",
        "../aoc-2025/day-01/input.txt"
    ).unwrap();

    assert_eq!(result, Value::Number(6770), "Day 1 Part 2 should return 6770");
}
