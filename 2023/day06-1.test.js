function parse(input) {
  const array = input
    .split("\n")
    .map((line) => line.split(":")[1].trim().split(/\s+/).map(Number));

  /** @see https://stackoverflow.com/a/17428705 */
  return array[0].map((_, colIndex) => array.map((row) => row[colIndex]));
}

function waysToBeatOneRecord([totalTime, totalDistance]) {
  let numberOfWays = 0;
  for (let holdTime = 1; holdTime <= totalTime; holdTime++) {
    const remainingTime = totalTime - holdTime;
    const distance = holdTime * remainingTime;
    if (distance > totalDistance) numberOfWays++;
  }
  return numberOfWays;
}

function waysToBeatTheRecord(input) {
  return input.reduce((a, c) => a * waysToBeatOneRecord(c), 1);
}

describe("day06-1", () => {
  const input = ["Time:      7  15   30", "Distance:  9  40  200"].join("\n");

  test("parse", () => {
    expect(parse(input)).toStrictEqual([
      [7, 9],
      [15, 40],
      [30, 200],
    ]);
  });

  test("waysToBeatOneRecord", () => {
    const parsed = parse(input);
    expect(waysToBeatOneRecord(parsed[0])).toBe(4);
    expect(waysToBeatOneRecord(parsed[1])).toBe(8);
    expect(waysToBeatOneRecord(parsed[2])).toBe(9);
  });

  test("waysToBeatTheRecord", () => {
    const parsed = parse(input);
    expect(waysToBeatTheRecord(parsed)).toBe(288);
  });
});
