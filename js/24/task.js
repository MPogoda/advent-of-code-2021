console.time("parser");
const filename = "input";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();
const data = rawData.map(row => row.split(" "));
console.timeEnd("parser");

console.time("prepare");
// each block is
// w = {INPUT[i]}
// …
// x = z % 26 + {A}
// …
// z = z / {B}
// …
// y = (x != w) * 25 + 1
// z *= y  // * 1 if x == w, 26 otherwise
// z += (w + {C}) * (x != w)
//
// C is always on line 15 of each 18-line block (e.g `add y 13`)

const As = data
  .filter(([op, lhs, rhs]) => op === "add" && lhs === "x" && rhs !== "z")
  .map(([, , rhs]) => Number(rhs));
const Bs = data
  .filter(([op, lhs]) => op === "div" && lhs === "z")
  .map(([, , rhs]) => Number(rhs));
const Cs = data
  .filter(([op, lhs], line) => op === "add" && lhs === "y" && line % 18 === 15)
  .map(([, , rhs]) => Number(rhs));

// as z can be decreased only by dividing by {Bs}, and not all Bs are > 1, we can avoid brute forcing when z cannot be successfully decreased to 0
const maxZs = Bs.map((_, i) => Bs.slice(i).reduce((acc, v) => acc * v, 1));
console.timeEnd("prepare");

console.time("Part 1+2");
(() => {
  function calculate(i, z, w) {
    const x = As[i] + (z % 26);
    z = Math.trunc(z / Bs[i]);
    if (x !== w) {
      z *= 26;
      z += w + Cs[i];
    }
    return z;
  }

  const DIGITS = "123456789".split("").map(Number);

  const cache = new Map();

  function solve(i, currentZ) {
    if (i === 14) {
      return currentZ === 0 ? [0] : [];
    }
    if (currentZ > maxZs[i]) return [];

    const result = [];

    for (const w of DIGITS) {
      const z = calculate(i, currentZ, w);
      let value = cache.get(`${i + 1}:${z}`);
      if (value === undefined) {
        value = solve(i + 1, z);
      }
      const ww = w * 10 ** (13 - i);
      for (const subResult of value) {
        result.push(ww + subResult);
      }
    }

    cache.set(`${i}:${currentZ}`, result);
    return result;
  }
  const results = solve(0, 0);
  results.sort((a, b) => a - b);

  console.log({
    min: results[0],
    max: results[results.length - 1],
    length: results.length
  });
})();
console.timeEnd("Part 1+2");
