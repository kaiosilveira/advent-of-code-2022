# Boiling Boulders

You and the elephants finally reach fresh air. You've emerged near the base of a large volcano that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and toward the ocean.

Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a little longer. Outside the cave, you can see the lava landing in a pond and hear it loudly hissing as it solidifies.

Depending on the specific compounds in the lava and speed at which it cools, it might be forming obsidian! The cooling rate should be based on the surface area of the lava droplets, so you take a quick scan of a droplet as it flies past you (your puzzle input).

Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low and, as a result, it approximates the shape of the lava droplet with **1x1x1 cubes on a 3D grid**, each given as its `x,y,z` position.

To approximate the surface area, count the number of sides of each cube that are not immediately connected to another cube. So, if your scan were only two adjacent cubes like `1,1,1` and `2,1,1`, each cube would have a single side covered and five sides exposed, a total surface area of **10** sides.

Here's a larger example:

```
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
```

In the above example, after counting up all the sides that aren't connected to another cube, the total surface area is **64**.

**What is the surface area of your scanned lava droplet?**

## Part I: part one title

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

In a test-driven development fashion, we can use the example provided in the text above as a test harness, to make sure that, for a given input, we're returning the correct output. We can make this test pass as fast as possible just by hard-coding its return value, so we're always on the green bar, and as soon we're able to replace the hard-coded values by real code, it means we're finished. The test harness extracted from the example is:

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn test_harness_for_part_one() {}
}
```

### Solution

```rust
pub fn part_one(_input: &Vec<&str>) {
  todo!();
}
```

</details>

---

## Part II: part two title

Something seems off about your calculation. The cooling rate depends on exterior surface area, but your calculation also included the surface area of air pockets trapped in the lava droplet.

Instead, consider only cube sides that could be reached by the water and steam as the lava droplet tumbles into the pond. The steam will expand to reach as much as possible, completely displacing any air on the outside of the lava droplet but never expanding diagonally.

In the larger example above, exactly one cube of air is trapped within the lava droplet (at `2,2,5`), so the exterior surface area of the lava droplet is 58.

**What is the exterior surface area of your scanned lava droplet?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

Similar to what we did for part one, the test harness for part two is:

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn test_harness_for_part_two() {}
}
```

### Solution

```rust
pub fn part_two(_input: &Vec<&str>) {
  todo!();
}
```

</details>
