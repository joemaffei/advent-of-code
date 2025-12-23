function parse(input) {
  const [instructions, nodes] = input.split("\n\n");
  const nodeMap = nodes.split("\n").reduce((map, line) => {
    const [node, L, R] = line.replace(/\s|\(|\)/g, "").split(/=|,/g);
    map[node] = { L, R };
    return map;
  }, {});

  return {
    instructions,
    nodeMap,
  };
}

function createInstructionGenerator() {
  return function* getInstruction(instructions) {
    let index = 0;
    while (true) {
      yield instructions.charAt(index);
      index++;
      if (index === instructions.length) index = 0;
    }
  };
}

function traverseMap({ instructions, nodeMap }) {
  const getInstruction = createInstructionGenerator();
  const instruction = getInstruction(instructions);
  let steps = 0;
  let currentNode = "AAA";
  while (true) {
    const direction = instruction.next().value;
    const node = nodeMap[currentNode];
    currentNode = node[direction];
    steps++;
    if (currentNode === "ZZZ") return steps;
  }
}

function result(input) {
  return traverseMap(parse(input));
}

// result($('pre').innerHTML.trim());

describe("day08-1", () => {
  const input1 = [
    "RL",
    "",
    "AAA = (BBB, CCC)",
    "BBB = (DDD, EEE)",
    "CCC = (ZZZ, GGG)",
    "DDD = (DDD, DDD)",
    "EEE = (EEE, EEE)",
    "GGG = (GGG, GGG)",
    "ZZZ = (ZZZ, ZZZ)",
  ].join("\n");

  const input2 = [
    "LLR",
    "",
    "AAA = (BBB, BBB)",
    "BBB = (AAA, ZZZ)",
    "ZZZ = (ZZZ, ZZZ)",
  ].join("\n");

  test("parse", () => {
    expect(parse(input1)).toStrictEqual({
      instructions: "RL",
      nodeMap: {
        AAA: { L: "BBB", R: "CCC" },
        BBB: { L: "DDD", R: "EEE" },
        CCC: { L: "ZZZ", R: "GGG" },
        DDD: { L: "DDD", R: "DDD" },
        EEE: { L: "EEE", R: "EEE" },
        GGG: { L: "GGG", R: "GGG" },
        ZZZ: { L: "ZZZ", R: "ZZZ" },
      },
    });

    expect(parse(input2)).toStrictEqual({
      instructions: "LLR",
      nodeMap: {
        AAA: { L: "BBB", R: "BBB" },
        BBB: { L: "AAA", R: "ZZZ" },
        ZZZ: { L: "ZZZ", R: "ZZZ" },
      },
    });
  });

  test("result", () => {
    expect(result(input1)).toBe(2);
    expect(result(input2)).toBe(6);
  });
});
