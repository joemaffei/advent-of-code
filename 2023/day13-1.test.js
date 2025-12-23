// TODO: rewrite this converting the lines to binary (how did I miss this?! lol)

function parse(input) {
  const result = input.split(/\n\n/g).map((block) => block.split(/\n/g));
  return result;
}

function createMirrors(order) {
  let result = [];
  const digits = [...Array(order - 1).keys()].map((x) => x + 1);
  const reverse = [...digits].reverse();
  const grandMirror = [...reverse, ...digits];
  for (let i = 1; i < order; i++) {
    const start = order - i - 1;
    const end = start + order;
    const mirror = grandMirror.slice(start, end);
    const noSingles = mirror.reduce((a, c, i, r) => {
      if (r.filter((x) => x === c).length === 1) {
        a.push(0);
      } else {
        a.push(+c);
      }
      return a;
    }, []);
    result.push(noSingles);
  }
  return result;
}

function matchMirror(str, mirror) {
  let posMatches = [];
  for (const pos of mirror) {
    if (pos === 0) continue;
    const [first, second] = [
      ...mirror.join("").matchAll(new RegExp(pos, "g")),
    ].map((x) => x.index);
    const match = JSON.stringify(str[first]) === JSON.stringify(str[second]);
    posMatches.push(match);
    if (pos === 1) return posMatches.every((match) => match === true);
  }
}

function findVerticalMirror(block) {
  const mirrors = createMirrors(block[0].length);
  for (const index in mirrors) {
    const mirror = mirrors[index];
    if (block.every((line) => matchMirror(line, mirror))) {
      return +index + 1;
    }
  }
  return -1;
}

function findHorizontalMirror(block) {
  const mirrors = createMirrors(block.length);
  for (const index in mirrors) {
    const mirror = mirrors[index];
    if (matchMirror(block, mirror)) {
      return +index + 1;
    }
  }
  return -1;
}

function answer(input) {
  const blocks = parse(input);
  let total = 0;
  for (const block of blocks) {
    const verticalMirror = findVerticalMirror(block);
    if (verticalMirror > -1) {
      total += verticalMirror;
      continue;
    }
    total += findHorizontalMirror(block) * 100;
  }
  return total;
}

// answer($("pre").innerHTML);

describe("day13-1", () => {
  const blocks = [
    [
      "#.##..##.",
      "..#.##.#.",
      "##......#",
      "##......#",
      "..#.##.#.",
      "..##..##.",
      "#.#.##.#.",
    ],
    [
      "#...##..#",
      "#....#..#",
      "..##..###",
      "#####.##.",
      "#####.##.",
      "..##..###",
      "#....#..#",
    ],
  ];

  const lines = [
    "#.##..##.",
    "..#.##.#.",
    "##......#",
    "##......#",
    "..#.##.#.",
    "..##..##.",
    "#.#.##.#.",
    "",
    "#...##..#",
    "#....#..#",
    "..##..###",
    "#####.##.",
    "#####.##.",
    "..##..###",
    "#....#..#",
  ];

  const input = lines.join("\n");

  test(parse.name, () => {
    expect(parse(input)).toStrictEqual(blocks);
  });

  test(createMirrors.name, () => {
    expect(createMirrors(5)).toStrictEqual([
      [1, 1, 0, 0, 0],
      [2, 1, 1, 2, 0],
      [0, 2, 1, 1, 2],
      [0, 0, 0, 1, 1],
    ]);
  });

  test(matchMirror.name, () => {
    expect(matchMirror("##...", [1, 1, 0, 0, 0])).toBe(true);
    expect(matchMirror("#.#..", [1, 1, 0, 0, 0])).toBe(false);
    expect(matchMirror(".##..", [2, 1, 1, 2, 0])).toBe(true);

    expect(matchMirror(["#.#", "#.#", "..."], [1, 1, 0])).toBe(true);
    expect(matchMirror(["#.#", "##.", "..."], [1, 1, 0])).toBe(false);
    expect(matchMirror(["...", "#.#", "#.#"], [0, 1, 1])).toBe(true);
  });

  test(findVerticalMirror.name, () => {
    expect(findVerticalMirror(blocks[0])).toBe(5);
  });

  test(findHorizontalMirror.name, () => {
    expect(findHorizontalMirror(blocks[1])).toBe(4);
  });

  test(answer.name, () => {
    expect(answer(input)).toBe(405);
  });
});
