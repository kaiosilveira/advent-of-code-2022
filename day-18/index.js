const fs = require("fs");

const parseInput = (input) => {
  return input.map((line, idx) => {
    const [x, y, z] = line.split(",");
    return { x: Number(x), y: Number(y), z: Number(z), id: idx };
  });
};

// const input = [
//   "2,2,2",
//   "1,2,2",
//   "3,2,2",
//   "2,1,2",
//   "2,3,2",
//   "2,2,1",
//   "2,2,3",
//   "2,2,4",
//   "2,2,6",
//   "1,2,5",
//   "3,2,5",
//   "2,1,5",
//   "2,3,5",
// ]; // 64
const input = fs.readFileSync("input.txt").toString().split("\n"); // 4364

function partOne(input) {
  const squarePositions = parseInput(input);
  const total = squarePositions
    .map((square) => {
      const xy = squarePositions.filter(
        (s) =>
          square.x === s.x &&
          square.y === s.y &&
          square.id !== s.id &&
          Math.abs(square.z - s.z) <= 1
      );
      const xz = squarePositions.filter(
        (s) =>
          square.x === s.x &&
          square.z === s.z &&
          square.id !== s.id &&
          Math.abs(square.y - s.y) <= 1
      );
      const yz = squarePositions.filter(
        (s) =>
          square.z === s.z &&
          square.y === s.y &&
          square.id !== s.id &&
          Math.abs(square.x - s.x) <= 1
      );

      let totalExposedSides = 6;

      if (xy.length > 0) totalExposedSides -= xy.length;
      if (xz.length > 0) totalExposedSides -= xz.length;
      if (yz.length > 0) totalExposedSides -= yz.length;

      return totalExposedSides;
    })
    .reduce((t, p) => t + p, 0);

  console.log(`Total: ${total}`);

  return total;
}

function partTwo(input) {
  const squarePositions = parseInput(input);
  const [xs, ys, zs] = squarePositions
    .map((sp) => [sp.x, sp.y, sp.z])
    .reduce(
      ([txs, tys, tzs], [px, py, pz]) => [
        [...txs, px],
        [...tys, py],
        [...tzs, pz],
      ],
      [[], [], []]
    );

  const [[minX, maxX], [minY, maxY], [minZ, maxZ]] = [
    [Math.min(...xs), Math.max(...xs)],
    [Math.min(...ys), Math.max(...ys)],
    [Math.min(...zs), Math.max(...zs)],
  ];

  let blockedCubeSidesCount = 0;
  for (let z = minZ; z <= maxZ; z++) {
    for (let y = minY; y <= maxY; y++) {
      for (let x = minX; x <= maxX; x++) {
        const neighbors = squarePositions.filter(
          (s) =>
            (s.x === x && s.y === y && Math.abs(z - s.z) === 1) ||
            (s.x === x && s.z === z && Math.abs(y - s.y) === 1) ||
            (s.y === y && s.z === z && Math.abs(x - s.x) === 1)
        );

        if (neighbors.length === 6) {
          blockedCubeSidesCount += 6;
          console.log(x, y, z, neighbors);
        }
      }
    }
  }

  console.log(
    `answer: ${
      squarePositions.length * 6 - blockedCubeSidesCount
    } | ${blockedCubeSidesCount}`
  );
}

partTwo(input);
