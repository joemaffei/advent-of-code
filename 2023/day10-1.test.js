function parse(input) {
  const lines = input.trim().split(/\n/g);
  const blankLine = ".".repeat(lines[0].length + 2);

  return [blankLine, ...lines.map((line) => `.${line}.`), blankLine].map(
    (line) => line.split("")
  );
}

function findStart(map) {
  for (const row in map) {
    for (const col in map[row]) {
      if (map[row][col] === "S") return [+col, +row];
    }
  }
}

function findInitialOutlets(map) {
  const [startCol, startRow] = findStart(map);
  const coordsToCheck = [
    [startCol, startRow - 1, "|F7", "up"],
    [startCol - 1, startRow, "-LF", "left"],
    [startCol + 1, startRow, "-J7", "right"],
    [startCol, startRow + 1, "|LJ", "down"],
  ];

  let outlets = [];
  for (const [nextCol, nextRow, validMoves, direction] of coordsToCheck) {
    if (validMoves.split("").includes(map[nextRow][nextCol])) {
      outlets.push([nextCol, nextRow, direction]);
    }
  }
  return outlets;
}

function computeNextMove([col, row, direction], char) {
  switch (char) {
    case "|":
      return direction === "up" ? [col, row - 1, "up"] : [col, row + 1, "down"];
    case "-":
      return direction === "left"
        ? [col - 1, row, "left"]
        : [col + 1, row, "right"];
    case "F":
      return direction === "up"
        ? [col + 1, row, "right"]
        : [col, row + 1, "down"];
    case "7":
      return direction === "up"
        ? [col - 1, row, "left"]
        : [col, row + 1, "down"];
    case "J":
      return direction === "right"
        ? [col, row - 1, "up"]
        : [col - 1, row, "left"];
    case "L":
      return direction === "down"
        ? [col + 1, row, "right"]
        : [col, row - 1, "up"];
    default:
      throw new Error(`invalid move: [${col},${row},${direction}], ${char}`);
  }
}

function loopLength(map) {
  const outlets = findInitialOutlets(map);
  const [startCol, startRow] = outlets[0];
  let length = 0;
  let char = map[startRow][startCol];
  let nextMove = outlets[0];
  while (char !== "S") {
    nextMove = computeNextMove(nextMove, char);
    length++;
    char = map[nextMove[1]][nextMove[0]];
  }
  return Math.ceil(length / 2);
}

function answer(input) {
  return loopLength(parse(input));
}

// answer($("pre").innerHTML);

describe("day10-1", () => {
  const input = ["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"].join("\n");

  test("parse", () => {
    expect(parse(input)).toStrictEqual([
      [".", ".", ".", ".", ".", ".", "."],
      [".", "7", "-", "F", "7", "-", "."],
      [".", ".", "F", "J", "|", "7", "."],
      [".", "S", "J", "L", "L", "7", "."],
      [".", "|", "F", "-", "-", "J", "."],
      [".", "L", "J", ".", "L", "J", "."],
      [".", ".", ".", ".", ".", ".", "."],
    ]);
  });

  test("findStart", () => {
    expect(findStart(parse(input))).toStrictEqual([1, 3]);
  });

  test("findInitialOutlets", () => {
    expect(findInitialOutlets(parse(input))).toStrictEqual([
      [2, 3, "right"],
      [1, 4, "down"],
    ]);
  });

  test("computeNextMove", () => {
    expect(computeNextMove([1, 1, "down"], "|")).toStrictEqual([1, 2, "down"]);
    expect(computeNextMove([1, 1, "up"], "|")).toStrictEqual([1, 0, "up"]);

    expect(computeNextMove([1, 1, "right"], "-")).toStrictEqual([
      2,
      1,
      "right",
    ]);
    expect(computeNextMove([1, 1, "left"], "-")).toStrictEqual([0, 1, "left"]);

    expect(computeNextMove([1, 1, "down"], "L")).toStrictEqual([2, 1, "right"]);
    expect(computeNextMove([1, 1, "left"], "L")).toStrictEqual([1, 0, "up"]);

    expect(computeNextMove([1, 1, "right"], "J")).toStrictEqual([1, 0, "up"]);
    expect(computeNextMove([1, 1, "down"], "J")).toStrictEqual([0, 1, "left"]);

    expect(computeNextMove([1, 1, "right"], "7")).toStrictEqual([1, 2, "down"]);
    expect(computeNextMove([1, 1, "up"], "7")).toStrictEqual([0, 1, "left"]);

    expect(computeNextMove([1, 1, "up"], "F")).toStrictEqual([2, 1, "right"]);
    expect(computeNextMove([1, 1, "left"], "F")).toStrictEqual([1, 2, "down"]);
  });

  test("answer", () => {
    expect(answer(input)).toBe(8);
  });
});
