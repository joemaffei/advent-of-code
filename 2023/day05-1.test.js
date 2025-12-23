function parse(lines) {
  const sections = lines.split(/\n\n/);
  const [seeds, ...maps] = sections.map((section) => section.split(/\n/));
  return {
    seeds: seeds[0].slice(7).split(" ").map(Number),
    maps: maps.map((map) => map.slice(1).map((l) => l.split(" ").map(Number))),
  };
}

function inRange(num, min, max) {
  return num >= min && num <= max;
}

function processMap(map, input) {
  for (const [dStart, sStart, length] of map) {
    if (inRange(input, sStart, sStart + length)) {
      return dStart - sStart + input;
    }
  }
  return input;
}

function processAllMaps(maps, seeds) {
  const locations = [];
  for (const seed of seeds) {
    let location = seed;
    for (const map of maps) {
      location = processMap(map, location);
    }
    locations.push(location);
  }
  return locations;
}

function lowestLocationNumber(lines) {
  const { maps, seeds } = parse(lines);
  const locations = processAllMaps(maps, seeds);
  return Math.min.apply(null, locations);
}

describe("day05-1", () => {
  const input = [
    "seeds: 79 14 55 13",
    "",
    "seed-to-soil map:",
    "50 98 2",
    "52 50 48",
    "",
    "soil-to-fertilizer map:",
    "0 15 37",
    "37 52 2",
    "39 0 15",
    "",
    "fertilizer-to-water map:",
    "49 53 8",
    "0 11 42",
    "42 0 7",
    "57 7 4",
    "",
    "water-to-light map:",
    "88 18 7",
    "18 25 70",
    "",
    "light-to-temperature map:",
    "45 77 23",
    "81 45 19",
    "68 64 13",
    "",
    "temperature-to-humidity map:",
    "0 69 1",
    "1 0 69",
    "",
    "humidity-to-location map:",
    "60 56 37",
    "56 93 4",
  ].join("\n");

  test("processMap", () => {
    const map = parse(input).maps[0];
    expect(processMap(map, 79)).toBe(81);
    expect(processMap(map, 14)).toBe(14);
    expect(processMap(map, 55)).toBe(57);
    expect(processMap(map, 13)).toBe(13);
  });

  test("processAllMaps", () => {
    const { maps, seeds } = parse(input);
    expect(processAllMaps(maps, seeds)).toStrictEqual([82, 43, 86, 35]);
  });

  test("lowestLocationNumber", () => {
    expect(lowestLocationNumber(input)).toBe(35);
  });
});
