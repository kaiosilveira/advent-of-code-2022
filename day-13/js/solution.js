const ordering = {
  GREATER: "greater",
  SMALLER: "smaller",
  UNKNOWN: "unknown",
};

function recursivelyCheckOrder(left, right) {
  let analysis = ordering.UNKNOWN;
  let allItemsChecked = false;

  if (left.length === 0 && right.length === 0) {
    return ordering.UNKNOWN;
  }

  while (analysis === ordering.UNKNOWN && !allItemsChecked) {
    let leftSideIsSmaller = left.length < right.length;
    for (let i = 0; i < left.length; i++) {
      const leftItem = left[i];
      const rightItem = right[i];

      allItemsChecked = i >= left.length - 1;
      if (rightItem === undefined) {
        analysis = ordering.GREATER;
        break;
      }

      if (typeof leftItem === "number" && typeof rightItem === "number") {
        if (leftItem < rightItem) {
          analysis = ordering.SMALLER;
          break;
        } else if (leftItem === rightItem) {
          continue;
        } else {
          analysis = ordering.GREATER;
          break;
        }
      } else if (Array.isArray(leftItem) && Array.isArray(rightItem)) {
        analysis = recursivelyCheckOrder(leftItem, rightItem);
        if (analysis != ordering.UNKNOWN || i == left.length - 1) {
          break;
        }
      } else if (!Number.isNaN(leftItem) && Array.isArray(rightItem)) {
        analysis = recursivelyCheckOrder([leftItem], rightItem);
        if (analysis != ordering.UNKNOWN) {
          break;
        }
      } else if (Array.isArray(leftItem) && !Number.isNaN(rightItem)) {
        analysis = recursivelyCheckOrder(leftItem, [rightItem]);
        if (analysis != ordering.UNKNOWN) {
          break;
        }
      }
    }

    if (analysis === ordering.UNKNOWN && leftSideIsSmaller) {
      analysis = ordering.SMALLER;
    }
  }
  return analysis;
}

function isOrdered(left, right) {
  const analysis = recursivelyCheckOrder(left, right);

  switch (analysis) {
    case ordering.SMALLER:
      return true;
    case ordering.GREATER:
      return false;
    case ordering.UNKNOWN:
    default:
      throw new Error("Ordering should be known at this point");
  }
}

module.exports = { isOrdered };
