console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawScanners = require("fs").readFileSync(filename, "UTF-8").split("\n\n");

const scanners = rawScanners.map((s) => {
    const lines = s.split("\n").slice(1);
    return lines.filter((row) => row !== "").map((row) => row.split(",").map(Number));
});
console.timeEnd("parser");

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
const beacons = new Set(scanners[0].map((row) => row.join(":")));
const scannerPositions = Array(N).fill([0,0,0]);
scannerPositions[0] = [0, 0, 0];

const matched = new Set([0]);
const rotations = scanners.map(
    (scanner) => rotates.map(
        (rotate) => signs.map(
            (sign) => scanner.map((p) => adjustPoint(p, rotate, sign))
        )
    )
);

console.time("Part 1");
(() => {
    while (matched.size !== N) {
        for (let i = 1; i < N; ++i) {
            if (matched.has(i)) continue;

            const knownBeacons = Array.from(beacons).map((b) => b.split(":").map(Number));

            for (const rotate in rotates) {
                let found = false;
                for (const sign in signs) {
                    const thisRotate = rotations[i][rotate][sign];
                    const isOverlap = {};
                    for (const thisBeacon of thisRotate) {
                        let found2 = false;
                        for (const knownBeacon of knownBeacons) {
                            const dyxz = [
                                knownBeacon[0] - thisBeacon[0],
                                knownBeacon[1] - thisBeacon[1],
                                knownBeacon[2] - thisBeacon[2]
                            ].join(":");
                            isOverlap[dyxz] = (isOverlap[dyxz] ?? 0) + 1;
                            if (isOverlap[dyxz] >= 12) {
                                found2 = true;
                                break;
                            }
                        }
                        if (found2) break;
                    }

                    for (const key in isOverlap) {
                        if (isOverlap[key] >= 12) {
                            const [dx, dy, dz] = key.split(":").map(Number);
                            scannerPositions[i] = [dx, dy, dz];
                            matched.add(i);

                            for (const [px, py, pz] of thisRotate) {
                                beacons.add([px + dx, py + dy, pz + dz].join(":"));
                            }

                            found = true;
                            break;
                        }
                    }
                    if (found) break;
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
    for (const [x1,y1,z1] of scannerPositions) {
        for (const [x2,y2,z2] of scannerPositions) {
            max = Math.max(
                max,
                Math.abs(x2 - x1) + Math.abs(y2 - y1) + Math.abs(z2 - z1)
            );
        }
    }

    console.log(max);
})();
console.timeEnd("Part 2");
