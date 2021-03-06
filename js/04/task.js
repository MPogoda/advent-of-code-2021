console.time("parser");
const filename = "input";
// const filename = "testinput";
const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

const numbers = rawData.shift().split(",");

rawData.shift();

const fields = [];
while (rawData.length) {
  const field = [];
  while (rawData[0]?.length) {
    field.push(
      rawData
        .shift()
        .split(" ")
        .filter(x => x.length)
    );
  }
  fields.push(field);
  rawData.shift();
}
console.timeEnd("parser");

function play(field, n) {
  for (const row of field) {
    for (let i = 0; i < row.length; ++i) {
      if (row[i] === n) {
        row[i] = "X";

        if (row.every(x => x === "X") || field.every(r => r[i] === "X")) {
          return true;
        }
      }
    }
  }

  return false;
}

function score(field, n) {
  const sum = field
    .flatMap(r => r.filter(x => x !== "X"))
    .reduce((acc, v) => acc + Number(v), 0);

  return { sum, n: Number(n), ans: sum * Number(n) };
}

console.time("Part 1");
(() => {
  const localFields = fields.map(f => f.map(row => Array.from(row)));
  for (const n of numbers) {
    for (const field of localFields) {
      if (play(field, n)) {
        console.log(score(field, n));
        return;
      }
    }
  }
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const localFields = fields.map(f => f.map(row => Array.from(row)));
  const completed = new Set();

  for (const n of numbers) {
    for (const field of localFields) {
      if (completed.has(field)) continue;
      if (play(field, n)) {
        completed.add(field);
        if (completed.size === localFields.length) {
          console.log(score(field, n));
          return;
        }
      }
    }
  }
})();
console.timeEnd("Part 2");
