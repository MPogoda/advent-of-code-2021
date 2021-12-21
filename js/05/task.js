console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

const data = rawData.map(line => {
  const [, x1, y1, x2, y2] = /^(\d+),(\d+) -> (\d+),(\d+)$/.exec(line);
  return { x1: Number(x1), y1: Number(y1), x2: Number(x2), y2: Number(y2) };
});
console.timeEnd("parser");

console.time("Part 1");
(() => {
  const field = new Map();
  for (const { x1, y1, x2, y2 } of data) {
    if (x1 === x2) {
      for (let y = Math.min(y1, y2); y <= Math.max(y1, y2); ++y) {
        field.set(`${y}-${x1}`, 1 + (field.get(`${y}-${x1}`) ?? 0));
      }
    }
    if (y1 === y2) {
      for (let x = Math.min(x1, x2); x <= Math.max(x1, x2); ++x) {
        field.set(`${y1}-${x}`, 1 + (field.get(`${y1}-${x}`) ?? 0));
      }
    }
  }

  const answer = Array.from(field.values()).filter(x => x > 1).length;
  console.log(answer);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const field = new Map();
  for (const { x1, y1, x2, y2 } of data) {
    if (x1 === x2) {
      for (let y = Math.min(y1, y2); y <= Math.max(y1, y2); ++y) {
        field.set(`${y}-${x1}`, 1 + (field.get(`${y}-${x1}`) ?? 0));
      }
    }
    if (y1 === y2) {
      for (let x = Math.min(x1, x2); x <= Math.max(x1, x2); ++x) {
        field.set(`${y1}-${x}`, 1 + (field.get(`${y1}-${x}`) ?? 0));
      }
    }
    if (x1 < x2 && y1 < y2 && y2 - y1 === x2 - x1) {
      for (let i = 0; i <= x2 - x1; ++i) {
        const x = x1 + i;
        const y = y1 + i;
        field.set(`${y}-${x}`, 1 + (field.get(`${y}-${x}`) ?? 0));
      }
    }
    if (x1 < x2 && y2 < y1 && y1 - y2 === x2 - x1) {
      for (let i = 0; i <= x2 - x1; ++i) {
        const x = x1 + i;
        const y = y1 - i;
        field.set(`${y}-${x}`, 1 + (field.get(`${y}-${x}`) ?? 0));
      }
    }
    if (x2 < x1 && y2 < y1 && y1 - y2 === x1 - x2) {
      for (let i = 0; i <= x1 - x2; ++i) {
        const x = x1 - i;
        const y = y1 - i;
        field.set(`${y}-${x}`, 1 + (field.get(`${y}-${x}`) ?? 0));
      }
    }
    if (x2 < x1 && y1 < y2 && y2 - y1 === x1 - x2) {
      for (let i = 0; i <= x1 - x2; ++i) {
        const x = x1 - i;
        const y = y1 + i;
        field.set(`${y}-${x}`, 1 + (field.get(`${y}-${x}`) ?? 0));
      }
    }
  }

  const answer = Array.from(field.values()).filter(x => x > 1).length;
  console.log(answer);
})();
console.timeEnd("Part 2");
