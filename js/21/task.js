console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");

const player1 = Number(rawData[0].split(": ")[1]);
const player2 = Number(rawData[1].split(": ")[1]);

console.timeEnd("parser");

console.time("Part 1");
(() => {
  let rolls = 0;
  function createDie() {
    let nextDie = 1;
    return () => {
      rolls += 1;
      const r = nextDie;
      nextDie += 1;
      if (nextDie > 100) nextDie = 1;

      return r;
    };
  }

  const roll = createDie();

  function move(pos) {
    return [1, 2, 3]
      .map(() => roll())
      .reduce((p, v) => 1 + ((p - 1 + v) % 10), pos);
  }

  const players = [
    { score: 0, pos: player1 },
    { score: 0, pos: player2 }
  ];
  let turn = 0;

  const WINNING = 1000;

  while (players[0].score < WINNING && players[1].score < WINNING) {
    const newPos = move(players[turn % 2].pos);
    players[turn % 2].pos = newPos;
    players[turn % 2].score += newPos;

    ++turn;
  }

  console.log(rolls * Math.min(players[0].score, players[1].score));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const WINNING = 21;
  const MOVES_COUNT = [
    [3, 1],
    [4, 3],
    [5, 6],
    [6, 7],
    [7, 6],
    [8, 3],
    [9, 1]
  ];

  function play(start) {
    const roundToWins = [];
    let state = new Map([[[start, 0].join(":"), 1]]);
    while (state.size) {
      let currentRoundWins = 0;

      const nextState = new Map();
      for (const [rawK, v] of state) {
        const [pos, score] = rawK.split(":").map(Number);
        for (const [moves, count] of MOVES_COUNT) {
          const newPos = 1 + ((pos + moves - 1) % 10);
          // const newPos = POSITION_NUM_MOVES_MAP[[pos, moves].join(":")];
          const newScore = score + newPos;
          const numGames = v * count;
          if (newScore >= WINNING) {
            currentRoundWins += numGames;
          } else {
            const key = [newPos, newScore].join(":");
            nextState.set(key, numGames + (nextState.get(key) ?? 0));
          }
        }
      }
      state = nextState;

      roundToWins.push(currentRoundWins);
    }

    return roundToWins;
  }

  const p1 = play(player1);
  const p2 = play(player2);
  let totalP1 = 0;
  let prod = 1;
  for (let i in p1) {
    totalP1 += p1[i] * prod;
    prod *= 27;
    prod -= p2[i];
  }
  let totalP2 = 0;
  prod = 1;
  for (let i in p2) {
    prod *= 27;
    prod -= p1[i];
    totalP2 += p2[i] * prod;
  }

  console.log(Math.max(totalP1, totalP2));
})();
console.timeEnd("Part 2");
