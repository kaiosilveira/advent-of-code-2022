const fs = require('fs');
const { isOrdered } = require('./solution');

function parseInput() {
  const input = fs
    .readFileSync('sample.txt')
    .toString()
    .split('\n')
    .map(l => (l === '' ? l : JSON.parse(l)));

  input.push('');

  const leftRightPairs = [];
  let pair = [];

  input.forEach(line => {
    if (line === '') {
      leftRightPairs.push([...pair]);
      pair = [];
    } else pair.push(line);
  });

  return leftRightPairs;
}

function partOne() {
  const leftRightPairs = parseInput();
  const orderedPairs = leftRightPairs
    .map(([left, right], idx) => (isOrdered(left, right) ? idx + 1 : -1))
    .filter(i => i > 0);

  const sumOfOrderedPairs = orderedPairs.reduce((t, p) => t + p, 0);
  console.log(`Sum of ordered pairs: ${sumOfOrderedPairs}`);
}

function partTwo() {
  const leftRightPairs = parseInput().flatMap(v => v);
  leftRightPairs.push([[2]], [[6]]);

  const sortedLists = leftRightPairs.sort((listA, listB) => (isOrdered(listA, listB) ? -1 : 1));
  const dividerPositions = sortedLists
    .map((list, idx) => (['[[2]]', '[[6]]'].includes(JSON.stringify(list)) ? idx + 1 : -1))
    .filter(i => i > 0);

  const decoderKey = dividerPositions.reduce((a, b) => a * b, 1);
  console.log(`Decoder key: ${decoderKey}`);
}

partOne();
partTwo();
