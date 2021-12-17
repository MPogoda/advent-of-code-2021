console.time("parser");
const filename = "input";
// const filename = "testinput";

const [rawData] = require("fs").readFileSync(filename, "UTF-8").split("\n");
const [x1, x2, y1, y2] = /^target area: x=(.+)\.\.(.+), y=(.+)\.\.(.+)$/
  .exec(rawData)
  .slice(1)
  .map(Number);
console.timeEnd("parser");

function fly(dx, dy) {
  let [x, y] = [0, 0];
  let maxY = 0;

  while (x <= x2 && y >= y1) {
    x += dx;
    y += dy;
    maxY = Math.max(maxY, y);

    dx = Math.max(0, dx - 1);
    --dy;

    if (x >= x1 && x <= x2 && y >= y1 && y <= y2) return maxY;
  }

  return null;
}

console.time("Part 1+2");
(() => {
  let max = 0;
  let count = 0;
  let iterations = 0;
  for (let dy = y1; dy < -y1; ++dy) {
    for (let dx = x2; dx >= Math.sqrt(x1); --dx) {
      ++iterations;
      const r = fly(dx, dy);
      if (r !== null) {
        ++count;
        max = Math.max(max, r);
      }
    }
  }

  console.log({ count, max, iterations });
})();
console.timeEnd("Part 1+2");
