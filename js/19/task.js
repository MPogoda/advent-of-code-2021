console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawScanners = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n\n");

const scanners = rawScanners.map(s => {
  const lines = s.split("\n").slice(1);
  return lines.filter(row => row !== "").map(row => row.split(",").map(Number));
});
console.timeEnd("parser");

console.time("prepare");
const rotates = [
  [0, 1, 2],
  [0, 2, 1],
  [1, 0, 2],
  [1, 2, 0],
  [2, 0, 1],
  [2, 1, 0]
];
const signs = [
  [1, 1, 1],
  [1, 1, -1],
  [1, -1, 1],
  [1, -1, -1],
  [-1, 1, 1],
  [-1, 1, -1],
  [-1, -1, 1],
  [-1, -1, -1]
];
function adjustPoint(point, [xPos, yPos, zPos], [signX, signY, signZ]) {
  return [signX * point[xPos], signY * point[yPos], signZ * point[zPos]];
}

const N = scanners.length;
const beacons = new Map(scanners[0].map(row => [row.join(":"), row]));
const scannerPositions = Array(N).fill([0, 0, 0]);
scannerPositions[0] = [0, 0, 0];

const matched = new Set([0]);
const rotations = scanners.map(scanner =>
  rotates.map(rotate =>
    signs.map(sign => scanner.map(p => adjustPoint(p, rotate, sign)))
  )
);
console.timeEnd("prepare");

console.time("Part 1");
(() => {
  while (matched.size !== N) {
    for (let i = 1; i < N; ++i) {
      if (matched.has(i)) continue;

      for (const rotate in rotates) {
        let found = null;
        for (const sign in signs) {
          const thisRotate = rotations[i][rotate][sign];
          const isOverlap = {};
          for (const thisBeacon of thisRotate) {
            for (const [, knownBeacon] of beacons) {
              const dx = knownBeacon[0] - thisBeacon[0];
              const dy = knownBeacon[1] - thisBeacon[1];
              const dz = knownBeacon[2] - thisBeacon[2];
              const key = dx * dx + dy * dy + dz * dz;

              isOverlap[key] = (isOverlap[key] ?? 0) + 1;
              if (isOverlap[key] >= 12) {
                found = [dx, dy, dz];
                break;
              }
            }
            if (found) break;
          }

          if (found) {
            const [dx, dy, dz] = found;
            scannerPositions[i] = [dx, dy, dz];
            matched.add(i);

            for (const [px, py, pz] of thisRotate) {
              const beacon = [px + dx, py + dy, pz + dz];
              beacons.set(beacon.join(":"), beacon);
            }

            break;
          }
        }
        if (found) break;
      }
    }
  }

  console.log(beacons.size);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  let max = 0;
  for (const [x1, y1, z1] of scannerPositions) {
    for (const [x2, y2, z2] of scannerPositions) {
      max = Math.max(
        max,
        Math.abs(x2 - x1) + Math.abs(y2 - y1) + Math.abs(z2 - z1)
      );
    }
  }

  console.log(max);
})();
console.timeEnd("Part 2");
