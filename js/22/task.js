console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

const commands = rawData.map(line => {
  const [
    ,
    state,
    x1,
    x2,
    y1,
    y2,
    z1,
    z2
  ] = /^(.+) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$/.exec(line);

  return {
    on: state === "on",
    x1: Number(x1),
    x2: Number(x2),
    y1: Number(y1),
    y2: Number(y2),
    z1: Number(z1),
    z2: Number(z2)
  };
});
console.timeEnd("parser");

console.time("Part 1");
(() => {
  const result = new Set();
  for (const { on, x1, x2, y1, y2, z1, z2 } of commands) {
    for (let x = Math.max(x1, -50); x <= Math.min(x2, 50); ++x) {
      for (let y = Math.max(y1, -50); y <= Math.min(y2, 50); ++y) {
        for (let z = Math.max(z1, -50); z <= Math.min(z2, 50); ++z) {
          const key = [x, y, z].join(":");
          if (on) result.add(key);
          else result.delete(key);
        }
      }
    }
  }

  console.log(result.size);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const cubes = new Map();
  function intersect(lhs, [rx1, rx2, ry1, ry2, rz1, rz2]) {
    const result = {
      x1: Math.max(lhs.x1, rx1),
      y1: Math.max(lhs.y1, ry1),
      z1: Math.max(lhs.z1, rz1),

      x2: Math.min(lhs.x2, rx2),
      y2: Math.min(lhs.y2, ry2),
      z2: Math.min(lhs.z2, rz2)
    };

    if (
      result.x1 <= result.x2 &&
      result.y1 <= result.y2 &&
      result.z1 <= result.z2
    ) {
      return [
        result.x1,
        result.x2,
        result.y1,
        result.y2,
        result.z1,
        result.z2
      ].join(":");
    }
    return null;
  }

  for (const { on, ...newCube } of commands) {
    for (const [rawCube, sign] of Array.from(cubes)) {
      const cube = rawCube.split(":").map(Number);
      const icube = intersect(newCube, cube);
      if (icube) {
        cubes.set(icube, (cubes.get(icube) ?? 0) - sign);
      }
    }
    if (on) {
      const key = [
        newCube.x1,
        newCube.x2,
        newCube.y1,
        newCube.y2,
        newCube.z1,
        newCube.z2
      ].join(":");
      cubes.set(key, (cubes.get(key) ?? 0) + 1);
    }
  }

  const ans = Array.from(cubes).reduce((acc, [rawCube, sign]) => {
    const [x1, x2, y1, y2, z1, z2] = rawCube.split(":").map(Number);
    return acc + sign * (x2 - x1 + 1) * (y2 - y1 + 1) * (z2 - z1 + 1);
  }, 0);
  console.log(ans);
})();
console.timeEnd("Part 2");
