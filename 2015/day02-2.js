$("pre")
  .innerHTML
  .trim()
  .split(/\n/g)
  .map((l) => l.split(/x/g).sort((a, b) => a - b))
  .reduce((t, [a, b, c]) => t + 2 * a + 2 * b + a * b * c, 0);
