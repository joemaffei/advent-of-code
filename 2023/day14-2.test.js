function parse(input) {
  return transpose(
    input
      .trim()
      .split(/\n/g)
      .map((line) => line.split(""))
  );
}

function transpose(array2d) {
  return array2d[0].map((_, colIndex) => array2d.map((row) => row[colIndex]));
}

function reverse(array2d) {
  return array2d.map((row) => row.reverse());
}

// function rotateClockwise(array2d) {
//   return reverse(transpose(array2d));
// }

function rotateCounterclockwise(array2d) {
  return transpose(reverse(array2d));
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
  let array2d = parse(input);
  const states = new Set();
  for (let i = 0; i < 19; i++) {
    array2d = rotateCounterclockwise(array2d).map((line) =>
      iterate(line.join("")).split("")
    );
    // const arrayAsString = array2d.toString();
    // if (states.has(arrayAsString)) {
    //   console.log(`cycle detected after ${i} iterations`);
    //   break;
    // }
    // states.add(arrayAsString);
  }
  return array2d
    .map((line) =>
      roundRockIndices(line.join("")).reduce(
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
    expect(parse(input)).toStrictEqual(lines.map((line) => line.split("")));
  });

  test(iterate.name, () => {
    expect(iterate("OO.O.O..##")).toBe("OOOO....##");
    expect(iterate("..#...O.#.")).toBe("..#O....#.");
  });

  test(roundRockIndices.name, () => {
    expect(roundRockIndices("OO.O.O..##")).toStrictEqual([1, 2, 4, 6]);
  });

  test(answer.name, () => {
    expect(answer(input)).toBe(64);
  });
});
