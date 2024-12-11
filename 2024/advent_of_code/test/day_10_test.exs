defmodule AdventOfCode.Day10Test do
  use ExUnit.Case
  import AdventOfCode.Day10

  test "Part 1" do
    input = """
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    """

    assert part1(input) === 36
  end
end
