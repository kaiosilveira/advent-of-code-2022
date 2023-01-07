const fs = require('fs');
const { partOne } = require('./solution/part-01');

const input = fs.readFileSync('input.txt').toString().split('\n');
console.log(`part 1: ${partOne(input)}`);
