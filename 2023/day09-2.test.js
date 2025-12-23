function parse(input) {
  return input.trim().split(/\n/g);
}

function processSequence(sequence) {
  return sequence.reduce((newSequence, current, index, original) => {
    if (index > 0) {
      newSequence.push(current - original[index - 1]);
    }
    return newSequence;
  }, []);
}

function buildSequenceHistory(sequence) {
  let history = [sequence];
  while (true) {
    const nextSequence = processSequence(history[history.length - 1]);
    history.push(nextSequence);
    if (nextSequence.every((element) => element === 0)) return history;
  }
}

function convertLineToSequence(line) {
  return line.split(" ").map(Number);
}

function nextValueInSequence(history) {
  return history.reverse().reduce((last, _, index) => {
    if (index === history.length - 1) return last;
    const nextSequence = history[index + 1];
    const newValue = nextSequence[0] - last;
    return newValue;
  }, 0);
}

function result(input) {
  const lines = parse(input);
  const histories = lines.map((line) =>
    buildSequenceHistory(convertLineToSequence(line))
  );
  return histories.reduce(
    (total, history) => total + nextValueInSequence(history),
    0
  );
}

// result($("pre").innerHTML);

describe("day09-2", () => {
  const lines = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
  const input = lines.join("\n");

  test("processSequence", () => {
    expect(processSequence(convertLineToSequence(lines[0]))).toStrictEqual([
      3, 3, 3, 3, 3,
    ]);
    expect(processSequence([3, 3, 3, 3, 3])).toStrictEqual([0, 0, 0, 0]);
  });

  test("buildSequenceHistory", () => {
    expect(buildSequenceHistory([10, 13, 16, 21, 30, 45])).toStrictEqual([
      [10, 13, 16, 21, 30, 45],
      [3, 3, 5, 9, 15],
      [0, 2, 4, 6],
      [2, 2, 2],
      [0, 0],
    ]);
  });

  test("convertLineToSequence", () => {
    expect(convertLineToSequence(lines[2])).toStrictEqual([
      10, 13, 16, 21, 30, 45,
    ]);
  });

  test("nextValueInSequence", () => {
    expect(
      nextValueInSequence(buildSequenceHistory(convertLineToSequence(lines[0])))
    ).toBe(-3);
    expect(
      nextValueInSequence(buildSequenceHistory(convertLineToSequence(lines[1])))
    ).toBe(0);
    expect(
      nextValueInSequence(buildSequenceHistory(convertLineToSequence(lines[2])))
    ).toBe(5);
  });

  test("result", () => {
    expect(result(input)).toBe(2);
  });
});
