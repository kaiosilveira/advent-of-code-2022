const { resolveExpressionFor } = require('../utils');

function partOne(input) {
  const monkeyYells = input.reverse().map(line => {
    const [monkeyName, expression] = line.split(': ');
    return { monkeyName, yell: expression };
  });

  const target = monkeyYells.find(m => m.monkeyName === 'root');

  return resolveExpressionFor(target, monkeyYells);
}

module.exports = { partOne };
