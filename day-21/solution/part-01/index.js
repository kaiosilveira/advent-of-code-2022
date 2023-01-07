function resolveExpressionFor(monkey, yells) {
  const parsingAttempt = parseInt(monkey.yell, 10);
  if (!isNaN(parsingAttempt)) return parsingAttempt;

  const [firstPart, operator, secondPart] = monkey.yell.split(' ');
  return eval(
    `${resolveExpressionFor(
      yells.find(m => m.monkeyName === firstPart),
      yells
    )} ${operator} ${resolveExpressionFor(
      yells.find(m => m.monkeyName === secondPart),
      yells
    )}`
  );
}

function partOne(input) {
  const monkeyYells = input.reverse().map(line => {
    const [monkeyName, expression] = line.split(': ');
    return { monkeyName, yell: expression };
  });

  const target = monkeyYells.find(m => m.monkeyName === 'root');

  return resolveExpressionFor(target, monkeyYells);
}

module.exports = { partOne, resolveExpressionFor };
