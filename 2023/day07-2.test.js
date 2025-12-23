function parse(input) {
  return input.split(/\n/).map((line) => line.split(" "));
}

function parseHand(cards) {
  return cards.split("").reduce((a, c) => {
    if (c in a) {
      a[c]++;
    } else {
      a[c] = 1;
    }
    return a;
  }, {});
}

function handScore(cards) {
  const cardsToBase14 = cards
    .replace(/T/g, "a")
    .replace(/Q/g, "b")
    .replace(/K/g, "c")
    .replace(/A/g, "d")
    .replace(/J/g, "1");
  const cardCounts = Object.values(parseHand(cards)).sort().join("");
  let rank = 0;
  if (cardCounts === "5") rank = 6;
  if (cardCounts === "14") rank = 5;
  if (cardCounts === "23") rank = 4;
  if (cardCounts === "113") rank = 3;
  if (cardCounts === "122") rank = 2;
  if (cardCounts === "1112") rank = 1;
  return parseInt(`${rank}${cardsToBase14}`, 14);
}

function sortLines(lines) {
  return [...lines].sort(([a], [b]) => {
    return handScore(a) - handScore(b);
  });
}

function answer(input) {
  const sorted = sortLines(parse(input));
  return sorted.reduce((total, [_, score], index) => {
    return total + +score * (index + 1);
  }, 0);
}

describe("day07-2", () => {
  const input = [
    "32T3K 765",
    "T55J5 684",
    "KK677 28",
    "KTJJT 220",
    "QQQJA 483",
  ].join("\n");

  test("answer", () => {
    expect(answer(input)).toBe(5905);
  });
});
