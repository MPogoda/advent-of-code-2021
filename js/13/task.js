console.time("parser");
const filename = "input";
// const filename = "testinput";

const [rawRows, rawFolds] = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n\n");

const rows = rawRows.split("\n");
const folds = rawFolds
  .split("\n")
  .slice(0, -1)
  .map(fold => {
    const [, dir, v] = /^fold along (.)=(\d+)$/.exec(fold);
    return { dir, v: Number(v) };
  });

const paperInput = new Set(rows);

console.timeEnd("parser");

function transform({ x, y }, { dir, v }) {
  if (dir === "y") {
    if (y < v) {
      return { x, y };
    } else if (y > v) {
      return { x, y: 2 * v - y };
    } else {
      return null;
    }
  } else {
    if (x < v) {
      return { x, y };
    } else if (x > v) {
      return { x: 2 * v - x, y };
    } else {
      return null;
    }
  }
}
function foldWith(paper, fold) {
  const result = new Set();
  for (const row of paper) {
    const [x, y] = row.split(",").map(Number);
    const coord = transform({ x, y }, fold);
    if (coord) {
      result.add(`${coord.x},${coord.y}`);
    }
  }
  return result;
}
console.time("Part 1");
(() => {
  const [fold] = folds;
  const newPaper = foldWith(paperInput, fold);
  console.log(newPaper.size);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const rPaper = folds.reduce((acc, fold) => foldWith(acc, fold), paperInput);

  const [W, H] = folds.reduce(
    ([w, h], { dir, v }) => [dir === "x" ? v : w, dir === "y" ? v : h],
    [Infinity, Infinity]
  );

  const field = Array(H)
    .fill(".")
    .map(() => Array(W).fill("."));

  for (const row of rPaper) {
    const [x, y] = row.split(",").map(Number);
    field[y][x] = "#";
  }
  console.log(field.map(row => row.join("")).join("\n"));
})();
console.timeEnd("Part 2");
