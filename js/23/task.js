console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n")
  .slice(1);

console.debug(rawData);

console.timeEnd("parser");

function hash(state) {
  return (
    state.sideRooms.map(room => room.join("")).join("|") +
    "|" +
    state.hallway.join("")
  );
}

const DESIRED = { A: 0, B: 1, C: 2, D: 3 };
const COSTS = { A: 1, B: 10, C: 100, D: 1000 };

function isFinal(state) {
  return (
    state.hallway.every(v => v === 0) &&
    state.sideRooms.every((room, j) => room.every(v => DESIRED[v] === j))
  );
}

function solve(state) {
  const queue = [];
  const energyTo = new Map();

  energyTo.set(hash(state), 0);
  queue.push([0, state]);
  while (queue.length) {
    const [cost, thisState] = queue.pop();

    if (isFinal(thisState)) {
      return cost;
    }
    if (cost > energyTo.get(hash(thisState) ?? Infinity)) continue;

    for (const [extraCost, nextState] of getNextStates(thisState)) {
      const nextCost = extraCost + cost;
      const nextHash = hash(nextState);
      if (nextCost < (energyTo.get(nextHash) ?? Infinity)) {
        energyTo.set(nextHash, nextCost);
        queue.push([nextCost, nextState]);
      }
    }

    queue.sort(([a], [b]) => b - a);
  }
}

function getNextStates(state) {
  const result = [];
  for (
    let hallwayIndex = 0;
    hallwayIndex < state.hallway.length;
    ++hallwayIndex
  ) {
    if (state.hallway[hallwayIndex] === 0) continue;
    const amphipod = state.hallway[hallwayIndex];

    const desired = DESIRED[amphipod];

    if (state.sideRooms[desired].some(v => DESIRED[v] !== desired)) continue;

    const targetHallway = (desired + 1) * 2;
    let canGo = true;
    const min = Math.min(hallwayIndex, targetHallway);
    const max = Math.max(hallwayIndex, targetHallway);
    for (let j = min; j < max; ++j) {
      if (j !== hallwayIndex && state.hallway[j] !== 0) {
        canGo = false;
        break;
      }
    }
    if (!canGo) continue;

    const steps =
      max - min + state.roomCapacity - state.sideRooms[desired].length;

    const cost = steps * COSTS[amphipod];
    const nextState = clone(state);
    nextState.hallway[hallwayIndex] = 0;
    nextState.sideRooms[desired].push(amphipod);

    result.push([cost, nextState]);
  }

  for (let room = 0; room < state.sideRooms.length; ++room) {
    if (!state.sideRooms[room].length) continue;
    if (state.sideRooms[room].every(v => DESIRED[v] === room)) continue;

    const hallwayIndex = (room + 1) * 2;
    for (let i = hallwayIndex - 1; i >= 0; --i) {
      if (state.hallway[i] !== 0) break;
      if ([2, 4, 6, 8].includes(i)) continue;

      const nextState = clone(state);
      const amphipod = nextState.sideRooms[room].pop();
      nextState.hallway[i] = amphipod;

      const steps =
        hallwayIndex -
        i +
        nextState.roomCapacity -
        nextState.sideRooms[room].length;
      const cost = steps * COSTS[amphipod];

      result.push([cost, nextState]);
    }
    for (let i = hallwayIndex + 1; i < state.hallway.length; ++i) {
      if (state.hallway[i] !== 0) break;
      if ([2, 4, 6, 8].includes(i)) continue;

      const nextState = clone(state);
      const amphipod = nextState.sideRooms[room].pop();
      nextState.hallway[i] = amphipod;

      const steps =
        i -
        hallwayIndex +
        nextState.roomCapacity -
        nextState.sideRooms[room].length;
      const cost = steps * COSTS[amphipod];

      result.push([cost, nextState]);
    }
  }

  return result;
}

function clone(state) {
  return {
    hallway: Array.from(state.hallway),
    sideRooms: state.sideRooms.map(room => Array.from(room)),
    roomCapacity: state.roomCapacity
  };
}
console.time("Part 1");
(() => {
  const state = {
    sideRooms: [3, 5, 7, 9].map(j => [rawData[2][j], rawData[1][j]]),
    hallway: new Array(11).fill(0),
    roomCapacity: 2
  };
  console.log(solve(state));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const state = {
    sideRooms: [3, 5, 7, 9].map(j => [rawData[2][j], rawData[1][j]]),
    hallway: new Array(11).fill(0),
    roomCapacity: 4
  };
  state.sideRooms[0].splice(1, 0, "D", "D");
  state.sideRooms[1].splice(1, 0, "B", "C");
  state.sideRooms[2].splice(1, 0, "A", "B");
  state.sideRooms[3].splice(1, 0, "C", "A");
  console.log(solve(state));
})();
console.timeEnd("Part 2");
