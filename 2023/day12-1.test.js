function parseLine(line) {
  const [state, list] = line.split(" ");
  const listAsArray = list.split(",").map(Number);
  const questionMarks = [...state.matchAll(/\?/g)];
  const questionMarkIndices = questionMarks.map((match) => match.index);
  return [state, questionMarkIndices, listAsArray];
}

function parse(input) {
  return input.trim().split(/\n/g).map(parseLine);
}

function binaries(order) {
  return [...Array(Math.pow(2, order)).keys()].map((n) =>
    n.toString(2).padStart(order, "0")
  );
}

function replaceAtIndex(str, index, replacement) {
  let arr = str.split("");
  arr[index] = replacement;
  return arr.join("");
}

function arrangements([state, occurrenceIndices, listAsArray]) {
  const perms = [];
  for (const binary of binaries(occurrenceIndices.length)) {
    let str = state;
    for (const i in occurrenceIndices) {
      const replacement = binary[i] === "0" ? "." : "#";
      str = replaceAtIndex(str, occurrenceIndices[i], replacement);
    }
    perms.push(str);
  }
  const regexBody = listAsArray.reduce((str, cur, index) => {
    return `${str}#{${cur}}\\.${index === listAsArray.length - 1 ? "*" : "+"}`;
  }, "");
  const regex = new RegExp(`^\\.*${regexBody}$`);
  return perms.filter((perm) => regex.test(perm));
}

function answer(input) {
  return parse(input).reduce((sum, args) => sum + arrangements(args).length, 0);
}

// answer($("pre").innerHTML);

describe("day12-1", () => {
  const lines = [
    "???.### 1,1,3",
    ".??..??...?##. 1,1,3",
    "?#?#?#?#?#?#?#? 1,3,1,6",
    "????.#...#... 4,1,1",
    "????.######..#####. 1,6,5",
    "?###???????? 3,2,1",
  ];

  const input = lines.join("\n");

  test(parseLine.name, () => {
    expect(parseLine(lines[0])).toStrictEqual([
      "???.###",
      [0, 1, 2],
      [1, 1, 3],
    ]);
    expect(parseLine(lines[1])).toStrictEqual([
      ".??..??...?##.",
      [1, 2, 5, 6, 10],
      [1, 1, 3],
    ]);
  });

  test(binaries.name, () => {
    expect(binaries(1)).toStrictEqual(["0", "1"]);
    expect(binaries(2)).toStrictEqual(["00", "01", "10", "11"]);
  });

  test(replaceAtIndex.name, () => {
    expect(replaceAtIndex("abcde", 2, "X")).toBe("abXde");
  });

  test(arrangements.name, () => {
    expect(arrangements(parseLine(lines[0]))).toHaveLength(1);
    expect(arrangements(parseLine(lines[1]))).toHaveLength(4);
    expect(arrangements(parseLine(lines[2]))).toHaveLength(1);
    expect(arrangements(parseLine(lines[3]))).toHaveLength(1);
    expect(arrangements(parseLine(lines[4]))).toHaveLength(4);
    expect(arrangements(parseLine(lines[5]))).toHaveLength(10);
  });

  test(answer.name, () => {
    expect(answer(input)).toBe(21);
  });
});
