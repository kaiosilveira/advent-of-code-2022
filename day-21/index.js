const { partOne } = require('./solution');
const fs = require('fs');

const input = fs.readFileSync('input.txt').toString().split('\n');
console.log(`part 1: ${partOne(input)}`);
