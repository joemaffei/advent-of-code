$("pre")
  .innerHTML.split("\n")
  .slice(0, -1)
  .reduce(
    (total, line) =>
      total +
      line.match(/\d/g).reduce((lineTotal, digit, index, arr) => {
        if (!index) lineTotal = +digit * 10;
        if (index === arr.length - 1) lineTotal += +digit;
        return lineTotal;
      }, 0),
    0
  );
