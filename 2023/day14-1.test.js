function parse(input) {
  return transpose(
    input
      .trim()
      .split(/\n/g)
      .map((line) => line.split(""))
  ).map((line) => line.join(""));
}

function transpose(array2d) {
  return array2d[0].map((_, colIndex) => array2d.map((row) => row[colIndex]));
}

function iterate(line) {
  const original = line;
  const newLine = line.replaceAll(".O", "O.");
  if (newLine === original) return newLine;
  return iterate(newLine);
}

function roundRockIndices(line) {
  return [...line.matchAll(/O/g)].map((x) => x.index + 1);
}

function answer(input) {
  return parse(input)
    .map((line) =>
      roundRockIndices(iterate(line).split("").reverse().join("")).reduce(
        (lineTotal, index) => lineTotal + index,
        0
      )
    )
    .reduce((total, lineTotal) => total + lineTotal, 0);
}

// answer($("pre").innerHTML);

describe("day14-1", () => {
  const lines = [
    "O....#....",
    "O.OO#....#",
    ".....##...",
    "OO.#O....O",
    ".O.....O#.",
    "O.#..O.#.#",
    "..O..#O..O",
    ".......O..",
    "#....###..",
    "#OO..#....",
  ];
  const input = lines.join("\n");

  test(parse.name, () => {
    expect(parse(input)).toStrictEqual([
      "OO.O.O..##",
      "...OO....O",
      ".O...#O..O",
      ".O.#......",
      ".#.O......",
      "#.#..O#.##",
      "..#...O.#.",
      "....O#.O#.",
      "....#.....",
      ".#.O.#O...",
    ]);
  });

  test(iterate.name, () => {
    expect(iterate("OO.O.O..##")).toBe("OOOO....##");
    expect(iterate("..#...O.#.")).toBe("..#O....#.");
  });

  test(roundRockIndices.name, () => {
    expect(roundRockIndices("OO.O.O..##")).toStrictEqual([1, 2, 4, 6]);
  });

  test(answer.name, () => {
    expect(answer(input)).toBe(136);
  });
});
