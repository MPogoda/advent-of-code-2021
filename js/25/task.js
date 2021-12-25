const { nextTick } = require("process");

console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

const H = rawData.length;
const W = rawData[0].length;

const toEast = new Set();
const toSouth = new Set();

for (let i = 0; i < H; ++i) {
  for (let j = 0; j < W; ++j) {
    const pos = i * W + j;
    if (rawData[i][j] === ">") {
      toEast.add(pos);
    } else if (rawData[i][j] === "v") {
      toSouth.add(pos);
    }
  }
}

console.timeEnd("parser");

console.time("Part 1");
(() => {
  function evolve({ toEast, toSouth }) {
    const newState = { toEast, toSouth };
    let hasMoved = false;
    const newEast = new Set();
    for (const pos of toEast) {
      const j = pos % W;
      const nextPos = j < W - 1 ? pos + 1 : pos - j;
      if (!newState.toEast.has(nextPos) && !newState.toSouth.has(nextPos)) {
        hasMoved = true;
        newEast.add(nextPos);
      } else {
        newEast.add(pos);
      }
    }

    newState.toEast = newEast;

    const newSouth = new Set();
    for (const pos of toSouth) {
      const i = Math.trunc(pos / W);
      const nextPos = i < H - 1 ? pos + W : pos % W;
      if (!newState.toEast.has(nextPos) && !newState.toSouth.has(nextPos)) {
        hasMoved = true;
        newSouth.add(nextPos);
      } else {
        newSouth.add(pos);
      }
    }

    newState.toSouth = newSouth;

    return [newState, hasMoved];
  }

  let step = 0;
  let state = { toEast, toSouth };
  let hasMoved = true;
  while (hasMoved) {
    ++step;
    [state, hasMoved] = evolve(state);
  }
  console.log(step);
})();
console.timeEnd("Part 1");
