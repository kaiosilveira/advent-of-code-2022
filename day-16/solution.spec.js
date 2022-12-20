const { parseValve } = require("./solution");

describe("parseValve", () => {
  it("should create a valve obj with multiple available tunnels from a string", () => {
    const line = "Valve BB has flow rate=13; tunnels lead to valves CC, AA";

    const result = parseValve(line);

    expect(result.label).toBe("BB");
    expect(result.rate).toBe(13);
    expect(result.leadsTo).toEqual(["CC", "AA"]);
  });

  it("should create a valve with a single available tunnel from a string", () => {
    const line = "Valve HH has flow rate=22; tunnel leads to valve GG";

    const result = parseValve(line);

    expect(result.label).toBe("HH");
    expect(result.rate).toBe(22);
    expect(result.leadsTo).toEqual(["GG"]);
  });
});
