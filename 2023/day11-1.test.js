function parse(input) {
  return input
    .trim()
    .split(/\n/g)
    .map((line) => line.split(""));
}

function blankRowIndices(map) {
  return map.reduce((indices, current, index) => {
    if (current.every((char) => char === ".")) indices.push(index);
    return indices;
  }, []);
}

function blankColumnIndices(map) {
  let indices = [];
  for (const col in map[0]) {
    let onlyDots = true;
    for (const row in map) {
      if (map[row][col] !== ".") onlyDots = false;
    }
    if (onlyDots) indices.push(+col);
  }
  return indices;
}

function duplicateBlankRows(map) {
  const indices = blankRowIndices(map);
  let newMap = [...map];
  const blankLine = ".".repeat(map[0].length).split("");
  let offset = 0;
  for (const index of indices) {
    newMap.splice(index + offset, 0, blankLine);
    offset++;
  }
  return newMap;
}

function duplicateBlankColumns(map) {
  const indices = blankColumnIndices(map);
  let newMap = map.map((row) => [...row]);
  for (const row of newMap) {
    let offset = 0;
    for (const index of indices) {
      row.splice(index + offset, 0, ".");
      offset++;
    }
  }
  return newMap;
}

function findGalaxies(map) {
  const galaxies = [];
  for (const row in map) {
    for (const column in map[row]) {
      const char = map[row][column];
      if (char === "#") galaxies.push([+column, +row]);
    }
  }
  return galaxies;
}

function distanceBetweenGalaxies([x1, y1], [x2, y2]) {
  return Math.abs(x1 - x2) + Math.abs(y1 - y2);
}

function sumOfDistancesBetweenGalaxies(galaxies) {
  let total = 0;
  for (const index in galaxies) {
    const current = galaxies[index];
    const otherGalaxies = galaxies.slice(+index + 1);
    for (const galaxy of otherGalaxies) {
      const distance = distanceBetweenGalaxies(current, galaxy);
      total += distance;
    }
  }
  return total;
}

function answer(input) {
  const map = duplicateBlankRows(duplicateBlankColumns(parse(input)));
  const galaxies = findGalaxies(map);
  return sumOfDistancesBetweenGalaxies(galaxies);
}

answer($("pre").innerHTML);

describe("day11-1", () => {
  const input = [
    "...#......",
    ".......#..",
    "#.........",
    "..........",
    "......#...",
    ".#........",
    ".........#",
    "..........",
    ".......#..",
    "#...#.....",
  ].join("\n");

  test("parse", () => {
    expect(parse(input)).toHaveLength(10);
    expect(parse(input)[0]).toHaveLength(10);
  });

  test("blankRowIndices", () => {
    const map = parse(input);
    expect(blankRowIndices(map)).toStrictEqual([3, 7]);
  });

  test("blankColumnIndices", () => {
    const map = parse(input);
    expect(blankColumnIndices(map)).toStrictEqual([2, 5, 8]);
  });

  test("duplicateBlankRows", () => {
    const map = parse(input);
    const indices = blankRowIndices(map);
    const newMap = duplicateBlankRows(map);
    expect(newMap).toHaveLength(map.length + indices.length);
    for (const index of indices) {
      expect(newMap[index].every((char) => char === "."));
      expect(newMap[index + 1].every((char) => char === "."));
    }
  });

  test("duplicateBlankColumns", () => {
    const map = parse(input);
    const indices = blankColumnIndices(map);
    const newMap = duplicateBlankColumns(map);
    for (const index of indices) {
      for (const row of newMap) {
        expect(row).toHaveLength(map[0].length + indices.length);
        // expect(row[index]).toBe(".");
      }
    }
  });

  test("findGalaxies", () => {
    const map = parse(input);
    expect(findGalaxies(map)).toStrictEqual([
      [3, 0],
      [7, 1],
      [0, 2],
      [6, 4],
      [1, 5],
      [9, 6],
      [7, 8],
      [0, 9],
      [4, 9],
    ]);
  });

  test("distanceBetweenGalaxies", () => {
    const map = duplicateBlankColumns(duplicateBlankRows(parse(input)));
    const galaxies = findGalaxies(map);
    expect(distanceBetweenGalaxies(galaxies[0], galaxies[6])).toBe(15);
    expect(distanceBetweenGalaxies(galaxies[2], galaxies[5])).toBe(17);
    expect(distanceBetweenGalaxies(galaxies[7], galaxies[8])).toBe(5);
  });

  test("answer", () => {
    expect(answer(input)).toBe(374);
  });
});
