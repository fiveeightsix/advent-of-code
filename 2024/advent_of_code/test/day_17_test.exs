defmodule AdventOfCode.Day17Test do
  use ExUnit.Case
  import AdventOfCode.Day17

  test "part 1" do
    input = """
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    """

    assert part1(input) === 1930
  end
end
