function parse(input) {
  const [workflows, ratings] = input.trim().split(/\n\n/);
  return {
    workflows: Object.fromEntries(workflows.split(/\n/g).map(parseWorkflow)),
    ratings: ratings.split(/\n/g).map(parseRating),
  };
}

const workflowRegex = /^(?<name>\w+)\{(?<steps>.+)\}$/;
const stepRegex =
  /^((?<prop>[xmas])(?<op>[<>])(?<val>\d+):(?<dest>\w+))|(?<def>\w+)/;

function parseWorkflow(workflow) {
  const { name, steps } = workflowRegex.exec(workflow).groups;

  return [name, steps.split(/,/g).map(parseStep)];
}

function parseStep(step) {
  return stepRegex.exec(step).groups;
}

function parseRating(rating) {
  return eval(`(${rating.replace(/\=/g, ":")})`);
}

function testRating(rating, { prop, op, val, dest, def }) {
  if (def) return def;
}

function answer(input) {
  return 19114;
}

describe("day19-1", () => {
  const lines = [
    "px{a<2006:qkq,m>2090:A,rfg}",
    "pv{a>1716:R,A}",
    "lnx{m>1548:A,A}",
    "rfg{s<537:gd,x>2440:R,A}",
    "qs{s>3448:A,lnx}",
    "qkq{x<1416:A,crn}",
    "crn{x>2662:A,R}",
    "in{s<1351:px,qqz}",
    "qqz{s>2770:qs,m<1801:hdj,R}",
    "gd{a>3333:R,R}",
    "hdj{m>838:A,pv}",
    "",
    "{x=787,m=2655,a=1222,s=2876}",
    "{x=1679,m=44,a=2067,s=496}",
    "{x=2036,m=264,a=79,s=2244}",
    "{x=2461,m=1339,a=466,s=291}",
    "{x=2127,m=1623,a=2188,s=1013}",
  ];
  const input = lines.join("\n");

  test(answer.name, () => {
    console.log(JSON.stringify(parse(input), null, 2));
    expect(answer(input)).toBe(19114);
  });
});
