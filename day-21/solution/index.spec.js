const { partOne } = require('.');

describe('challenge parts', () => {
  it('part 1', () => {
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
