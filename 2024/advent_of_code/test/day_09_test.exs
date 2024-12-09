defmodule AdventOfCode.Day09Test do
  use ExUnit.Case
  import AdventOfCode.Day09

  test "part 1" do
    input = "2333133121414131402"

    assert part1(input) === 1928
  end
end
