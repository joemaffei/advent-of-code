function parse(input) {
  return input
    .split("\n")
    .map((line) => line.split(":")[1].replace(/\s+/g, ""))
    .map(Number);
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

describe("day06-1", () => {
  const input = ["Time:      7  15   30", "Distance:  9  40  200"].join("\n");

  test("parse", () => {
    expect(parse(input)).toStrictEqual([71530, 940200]);
  });

  test("waysToBeatOneRecord", () => {
    const parsed = parse(input);
    expect(waysToBeatOneRecord(parsed)).toBe(71503);
  });
});
