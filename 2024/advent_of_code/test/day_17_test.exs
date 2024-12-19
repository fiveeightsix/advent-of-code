defmodule AdventOfCode.Day17Test do
  use ExUnit.Case
  import AdventOfCode.Day17

  test "opcode 2: instruction bst" do
    registers = %{"A" => 0, "B" => 0, "C" => 9}
    result = instruction_bst(registers, 6)

    assert result === %{"A" => 0, "B" => 1, "C" => 9}
  end
  
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
