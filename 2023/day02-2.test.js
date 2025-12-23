function parseLine(line) {
  const [head, ...tail] = line.split(":");
  const gameId = head.slice(5);
  const subsets = tail
    .map((x) =>
      x
        .split(";")
        .map((x) =>
          Object.fromEntries(
            x.split(",").map((x) => x.trim().split(" ").reverse())
          )
        )
    )
    .flat();
  const mins = subsets.reduce(
    (a, c) => {
      return {
        ...a,
        red: c?.red ? Math.max(a.red, c.red) : a.red,
        green: c?.green ? Math.max(a.green, c.green) : a.green,
        blue: c?.blue ? Math.max(a.blue, c.blue) : a.blue,
      };
    },
    { red: 0, green: 0, blue: 0 }
  );
  return Object.values(mins).reduce((a, c) => a * c, 1);
}

function fn(line) {
  return parseLine(line);
}

describe("fn", () => {
  test("cases", () => {
    expect(fn("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")).toBe(
      48
    );
    expect(
      fn("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
    ).toBe(12);
    expect(
      fn(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
      )
    ).toBe(1560);
    expect(
      fn(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
      )
    ).toBe(630);
    expect(fn("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")).toBe(
      36
    );
  });
});
