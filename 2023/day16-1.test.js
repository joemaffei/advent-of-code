function parse(input) {
  return input
    .trim()
    .split(/\n/g)
    .map((line) => line.split(""));
}

function filterInvalidMoves(moves, array2d) {
  return moves.filter(
    ([x, y]) => x >= 0 && y >= 0 && x < array2d[0].length && y < array2d.length
  );
}

function nextMoves([x, y, dx, dy], array2d) {
  const sym = array2d[y][x];
  if (sym === "." || (sym === "|" && dy) || (sym === "-" && dx)) {
    return filterInvalidMoves([[x + dx, y + dy, dx, dy]], array2d);
  }
  if (sym === "|") {
    return filterInvalidMoves(
      [
        [x, y - 1, 0, -1],
        [x, y + 1, 0, 1],
      ],
      array2d
    );
  }
  if (sym === "-") {
    return filterInvalidMoves(
      [
        [x - 1, y, -1, 0],
        [x + 1, y, 1, 0],
      ],
      array2d
    );
  }
  if (sym === "/") {
    return filterInvalidMoves([[x - dy, y - dx, -dy, -dx]], array2d);
  }
  if (sym === "\\") {
    return filterInvalidMoves([[x + dy, y + dx, dy, dx]], array2d);
  }
}

function answer(input) {
  const array2d = parse(input);
  const visited = array2d.map((line) => line.map((_) => "."));
  let beams = [[0, 0, 1, 0]];
  visited[0][0] = "#";
  const states = new Set();
  while (true) {
    let newBeams = [];
    for (const beam of beams) {
      const [x, y] = beam;
      visited[y][x] = "#";
      const moves = nextMoves(beam, array2d);
      newBeams = newBeams.concat(moves);
    }
    const newState = visited.map((line) => line.join("")).join("");
    // if (states.has(newState) || newBeams.length === 0) break;
    if (states.has(newState)) {
      const total = visited.reduce(
        (total, line) => total + line.join("").replace(/\./g, "").length,
        0
      );
      console.log({ total });
    }
    states.add(newState);
    beams = newBeams;
  }
  // console.log(visited.map((x) => x.join("")).join("\n"));
  // return visited.reduce(
  //   (total, line) => total + line.join("").replace(/\./g, "").length,
  //   0
  // );
}

describe("day16-1", () => {
  const lines = [
    ".|...\\....",
    "|.-.\\.....",
    ".....|-...",
    "........|.",
    "..........",
    ".........\\",
    "..../.\\\\..",
    ".-.-/..|..",
    ".|....-|.\\",
    "..//.|....",
  ];
  const input = lines.join("\n");

  test(answer.name, () => {
    expect(answer(input)).toBe(46);
  });
});
