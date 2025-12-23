function parse(lines) {
  const sections = lines.split(/\n\n/);
  const [originalSeeds, ...maps] = sections.map((section) =>
    section.split(/\n/)
  );
  const parsedSeeds = originalSeeds[0].slice(7).split(" ").map(Number);
  const pairs = processPairs(parsedSeeds);
  return {
    pairs,
    maps: maps.map((map) => map.slice(1).map((l) => l.split(" ").map(Number))),
  };
}

function createRange(start, length) {
  return Array.from(Array(length).keys(), (x) => x + start);
}

function processPairs(seeds) {
  const pairs = seeds.reduce((result, current, index) => {
    index % 2 ? result.at(-1).push(current) : result.push([current]);
    return result;
  }, []);
  return pairs;
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

function lowestLocationNumber(lines) {
  const { maps, pairs } = parse(lines);
  let lowestNumber = Infinity;
  for (const [start, length] of pairs) {
    console.log({ start, length });
    for (let location = start; location < start + length; location++) {
      if ((location - start) % 1e6 === 0) {
        console.log(
          `processed ${location - start}, ${start + length - location} to go`
        );
      }
      let currentLocation = location;
      for (const map of maps) {
        currentLocation = processMap(map, currentLocation);
      }
      if (currentLocation < lowestNumber) {
        lowestNumber = currentLocation;
        console.log({ lowestNumber });
      }
    }
  }
  return lowestNumber;
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

  test("lowestLocationNumber", () => {
    expect(lowestLocationNumber(input)).toBe(46);
  });
});
