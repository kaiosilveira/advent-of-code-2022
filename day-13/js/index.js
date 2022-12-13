const fs = require("fs");
const { isOrdered } = require("./solution");

function parseInput() {
  const input = fs
    .readFileSync("sample.txt")
    .toString()
    .split("\n")
    .map((l) => (l === "" ? l : JSON.parse(l)));

  input.push("");

  const leftRightPairs = [];
  let pair = [];

  input.forEach((line) => {
    if (line === "") {
      leftRightPairs.push([...pair]);
      pair = [];
    } else {
      pair.push(line);
    }
  });

  return leftRightPairs;
}

function partOne() {
  const leftRightPairs = parseInput();
  const orderedPairs = [];
  leftRightPairs.forEach(([left, right], idx) => {
    if (isOrdered(left, right)) orderedPairs.push(idx + 1);
  });

  console.log(
    `sum of ordered pairs: ${orderedPairs.reduce((t, p) => t + p, 0)}`
  );
}

function partTwo() {
  const leftRightPairs = parseInput().flatMap((v) => v);
  leftRightPairs.push(...[[[2]], [[6]]]);

  const updated = leftRightPairs.sort((arrA, arrB) =>
    isOrdered(arrA, arrB) ? -1 : 1
  );

  let dividerPositions = [];
  updated.forEach((arr, idx) => {
    if (["[[2]]", "[[6]]"].includes(JSON.stringify(arr))) {
      dividerPositions.push(idx + 1);
    }
  });

  console.log(
    `sum of decoder keys: ${dividerPositions.reduce((a, b) => a * b, 1)}`
  );
}

partOne();
partTwo();
