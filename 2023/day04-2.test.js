function intersect(arr1, arr2) {
  return arr1.filter((x) => arr2.includes(x));
}

function parse(line) {
  const parsed = line
    .split(/[:|]/)
    .map((l) => l.trim().replace(/\s+/g, " ").split(" "));
  return {
    id: +parsed[0][1],
    mine: parsed[1].map(Number),
    dealer: parsed[2].map(Number),
  };
}

function wins({ mine, dealer }) {
  const intersection = intersect(mine, dealer);
  return intersection.length;
}

function fn(lines) {
  const cards = lines.map((line) => {
    const card = parse(line);
    const numberOfWins = wins(card);
    card.wins = numberOfWins;
    card.score = numberOfWins ? Math.pow(2, numberOfWins - 1) : 0;
    card.copies = 1;
    return card;
  });
  console.log(cards);
}

describe("day04-2", () => {
  test("wins", () => {
    expect(
      wins(parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"))
    ).toBe(4);
    expect(
      wins(parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"))
    ).toBe(2);
    expect(
      wins(parse("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"))
    ).toBe(2);
    expect(
      wins(parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"))
    ).toBe(1);
    expect(
      wins(parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"))
    ).toBe(0);
    expect(
      wins(parse("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"))
    ).toBe(0);
  });
});
