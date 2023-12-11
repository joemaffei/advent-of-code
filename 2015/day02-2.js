$("pre")
  .innerHTML
  .trim()
  .split(/\n/g)
  .map((l) => l.split(/x/g).sort((a, b) => a - b))
  .reduce((t, [a, b, c]) => t + 2 * a * b + 2 * b * c + 2 * a * c + a * b, 0);
