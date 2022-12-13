const { partOne, partTwo } = require('./solution');
const fs = require('fs');
let input = fs.readFileSync('sample.txt').toString().split('\n');

console.log(`Part I: ${partOne(input)}`);
console.log(`Part II: ${partTwo(input)}`);
