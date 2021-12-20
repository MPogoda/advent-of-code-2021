console.time("parser");
const filename = "input";
// const filename = "testinput";

const [alg, rawData] = require("fs").readFileSync(filename, "UTF-8").split("\n\n");

const rawImg = rawData.split("\n");
rawImg.pop();

const image = {};
[image.h, image.w] = [rawImg.length, rawImg[0].length];

image.data = new Set();
for (let i = 0; i < image.h; ++i) {
    for (let j = 0; j < image.w; ++j) {
        if (rawImg[i][j] === '#') {
            image.data.add(`${i}:${j}`);
        }
    }
}

console.timeEnd("parser");

function evolve({h, w, data}, N) {
    h += N;
    w += N;

    for (let i = 0; i < N; ++i) {
        const nextData = new Set();
        for (let y = -N; y < h; ++y) {
            for (let x = -N; x < w; ++x) {
                let value = 0;
                for (const y2 of [y-1, y, y+1]) {
                    for (const x2 of [x-1, x, x+1]) {
                        value <<= 1;
                        if (x2 < -N || x2 > w-1 || y2 < -N || y2 > h-1) {
                            if (i%2 === 1 && alg[0] === '#') {
                                value += 1;
                            }
                        } else if (data.has(`${y2}:${x2}`)) {
                            value += 1;
                        }
                    }
                }
                if (alg[value] === '#') {
                    nextData.add(`${y}:${x}`);
                }
            }
        }

        data = nextData;
    }

    return data;
}

console.time("Part 1");
(() => {
    const data = evolve(image, 2);

    console.log(data.size);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    const data = evolve(image, 50);

    console.log(data.size);
})();
console.timeEnd("Part 2");
