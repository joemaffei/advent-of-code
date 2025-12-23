const regex = /(\d|one|two|three|four|five|six|seven|eight|nine)/;

const digitStringValues = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};

function fn(str) {
  /** @see https://stackoverflow.com/a/33903830 */
  const matches = Array.from(
    str.matchAll(
      /(?=(\d|one|two|three|four|five|six|seven|eight|nine))/g,
      (x) => x[1]
    )
  ).flat();

  return matches
    .map((match) =>
      match in digitStringValues ? digitStringValues[match] : +match
    )
    .filter(Number)
    .reduce((lineTotal, digit, index, arr) => {
      if (!index) lineTotal = +digit * 10;
      if (index === arr.length - 1) lineTotal += +digit;
      return lineTotal;
    }, 0);
}

describe("fn", () => {
  test("cases", () => {
    /**
     * edge cases
     * @see https://www.reddit.com/r/adventofcode/comments/1884fpl/2023_day_1for_those_who_stuck_on_part_2/
     */
    expect(fn("sevenine")).toBe(79);
    expect(fn("eighthree")).toBe(83);

    /**
     * spec cases
     */
    expect(fn("two1nine")).toBe(29);
    expect(fn("eightwothree")).toBe(83);
    expect(fn("abcone2threexyz")).toBe(13);
    expect(fn("xtwone3four")).toBe(24);
    expect(fn("4nineeightseven2")).toBe(42);
    expect(fn("zoneight234")).toBe(14);
    expect(fn("7pqrstsixteen")).toBe(76);
  });
});
