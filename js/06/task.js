console.time("parser");
const filename = "input";
// const filename = "testinput";
const PERIOD = 7;

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

const data = rawData[0].split(",");
console.timeEnd("parser");

function initialState(ds) {
  const state = {};
  for (const d of ds) {
    state[d] = 1 + (state[d] ?? 0);
  }
  return state;
}
function computeState(state) {
  return Object.entries(state).reduce((acc, [k, v]) => {
    if (k === "0") {
      acc[PERIOD - 1] = v + (acc[PERIOD - 1] ?? 0);
      acc[PERIOD + 1] = v;
    } else {
      acc[Number(k) - 1] = v + (acc[Number(k) - 1] ?? 0);
    }
    return acc;
  }, {});
}

console.time("Part 1");
(() => {
  let state = initialState(data);
  for (let i = 0; i < 80; ++i) {
    state = computeState(state);
  }
  console.log(Array.from(Object.values(state)).reduce((acc, v) => acc + v, 0));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  let state = initialState(data);
  for (let i = 0; i < 256; ++i) {
    state = computeState(state);
  }
  console.log(Array.from(Object.values(state)).reduce((acc, v) => acc + v, 0));
})();
console.timeEnd("Part 2");
