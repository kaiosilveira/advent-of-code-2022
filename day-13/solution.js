const ordering = {
  GREATER: 'greater',
  SMALLER: 'smaller',
  UNKNOWN: 'unknown',
};

function recursivelyCheckOrder(left, right) {
  if (left.length === 0 && right.length === 0) return ordering.UNKNOWN;

  const leftSideIsSmaller = left.length < right.length;

  let allItemsChecked = false;
  let analysis = ordering.UNKNOWN;

  while (analysis === ordering.UNKNOWN && !allItemsChecked) {
    for (let i = 0; i < left.length; i++) {
      allItemsChecked = i >= left.length - 1;
      const leftItem = left[i];
      const rightItem = right[i];

      if (rightItem === undefined) {
        analysis = ordering.GREATER;
        break;
      }

      if (Number.isInteger(leftItem) && Number.isInteger(rightItem)) {
        if (leftItem === rightItem) continue;
        analysis = leftItem < rightItem ? ordering.SMALLER : ordering.GREATER;
        break;
      }

      if (Array.isArray(leftItem) && Array.isArray(rightItem))
        analysis = recursivelyCheckOrder(leftItem, rightItem);
      else if (Number.isInteger(leftItem) && Array.isArray(rightItem))
        analysis = recursivelyCheckOrder([leftItem], rightItem);
      else if (Array.isArray(leftItem) && Number.isInteger(rightItem))
        analysis = recursivelyCheckOrder(leftItem, [rightItem]);

      if (analysis != ordering.UNKNOWN) break;
    }

    if (analysis === ordering.UNKNOWN && leftSideIsSmaller) analysis = ordering.SMALLER;
  }

  return analysis;
}

function isOrdered(left, right) {
  switch (recursivelyCheckOrder(left, right)) {
    case ordering.SMALLER:
      return true;
    case ordering.GREATER:
      return false;
    case ordering.UNKNOWN:
    default:
      throw new Error('Ordering should be known at this point');
  }
}

module.exports = { isOrdered };
