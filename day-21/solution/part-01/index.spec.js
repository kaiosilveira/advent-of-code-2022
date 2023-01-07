const { partOne, resolveExpressionFor } = require('.');

describe('resolveExpressionFor', () => {
  it('should return the resolved value of an expression if that is the case', () => {
    const monkey = { monkeyName: 'm1', yell: '3' };
    const yells = [monkey];

    const result = resolveExpressionFor(monkey, yells);

    expect(result).toEqual(3);
  });

  it('should resolve each expression part if it is a complex unresolved expression', () => {
    const monkey1 = { monkeyName: 'm1', yell: 'm2 + m3' };
    const monkey2 = { monkeyName: 'm2', yell: '3' };
    const monkey3 = { monkeyName: 'm3', yell: '4' };
    const yells = [monkey1, monkey2, monkey3];

    const result = resolveExpressionFor(monkey1, yells);

    expect(result).toEqual(7);
  });
});

describe('part 1', () => {
  it('should match the expected output', () => {
    const input = [
      'root: pppw + sjmn',
      'dbpl: 5',
      'cczh: sllz + lgvd',
      'zczc: 2',
      'ptdq: humn - dvpt',
      'dvpt: 3',
      'lfqf: 4',
      'humn: 5',
      'ljgn: 2',
      'sjmn: drzm * dbpl',
      'sllz: 4',
      'pppw: cczh / lfqf',
      'lgvd: ljgn * ptdq',
      'drzm: hmdt - zczc',
      'hmdt: 32',
    ];

    const result = partOne(input);

    expect(result).toEqual(152);
  });
});
