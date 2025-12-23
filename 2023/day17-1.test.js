function parse(input) {
  return input
    .trim()
    .split(/\n/g)
    .map((line) => line.split("").map(Number));
}

function answer(input) {
  return 102;
}

describe("day17-1", () => {
  const lines = [
    "2413432311323",
    "3215453535623",
    "3255245654254",
    "3446585845452",
    "4546657867536",
    "1438598798454",
    "4457876987766",
    "3637877979653",
    "4654967986887",
    "4564679986453",
    "1224686865563",
    "2546548887735",
    "4322674655533",
  ];
  const input = lines.join("\n");

  test(answer.name, () => {
    expect(answer(input)).toBe(102);
  });
});
