function fn(lines) {
  const paddingLine = `.${lines[0].replace(/./g, ".")}.`;
  const arr = [
    paddingLine,
    ...lines.map((line) => `.${line}.`),
    paddingLine,
  ].map((line) => line.split(""));
  const matches = [];
  for (const row in arr) {
    let start = 0;
    let end = 0;
    for (const col in arr[row]) {
      if (/\d/.test(arr[row][col])) {
        if (start === 0) {
          start = +col;
        }
      } else {
        if (start !== 0 && end === 0) {
          end = +col - 1;
        }
      }
      if (start && end) {
        matches.push({
          row: +row,
          start,
          end,
          value: arr[row].join("").substr(start, end - start + 1),
        });
        start = 0;
        end = 0;
      }
    }
  }
  let hits = [];
  for (const match of matches) {
    let positionsToCheck = [];
    for (let i = match.start - 1; i < match.end + 2; i++) {
      positionsToCheck.push(arr[match.row - 1][i]);
    }
    positionsToCheck.push(arr[match.row][match.start - 1]);
    positionsToCheck.push(arr[match.row][match.end + 1]);
    for (let i = match.start - 1; i < match.end + 2; i++) {
      positionsToCheck.push(arr[match.row + 1][i]);
    }
    if (positionsToCheck.some((char) => char !== ".")) {
      hits.push(+match.value);
    }
  }
  return hits.reduce((a, c) => a + c, 0);
}

describe("fn", () => {
  test("cases", () => {
    expect(
      fn([
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
      ])
    ).toBe(4361);
  });
});
