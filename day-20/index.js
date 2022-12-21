const input = [1, 2, -3, 3, -2, 0, 4];

const result = [];
input.forEach((n, idx) => {
  result[(idx + n) % input.length] = n;
});

console.log(result);
