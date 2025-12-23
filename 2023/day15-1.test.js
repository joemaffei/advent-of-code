function hash(str) {
  return str.split("").reduce((hash, char) => {
    // Determine the ASCII code for the current character of the string.
    const asciiCode = char.charCodeAt(0);
    // Increase the current value by the ASCII code you just determined.
    let newHash = hash + asciiCode;
    // Set the current value to itself multiplied by 17.
    newHash = newHash * 17;
    // Set the current value to the remainder of dividing itself by 256.
    newHash = newHash % 256;
    return newHash;
  }, 0);
}

function answer(input) {
  return input
    .trim()
    .split(",")
    .reduce((sum, command) => {
      return sum + hash(command);
    }, 0);
}

// answer($("pre").innerHTML);

describe("day15-1", () => {
  const input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

  test(hash.name, () => {
    expect(hash("HASH")).toBe(52);
  });

  test(answer.name, () => {
    expect(answer(input)).toBe(1320);
  });
});
