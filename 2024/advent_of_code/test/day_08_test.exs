defmodule AdventOfCode.Day08Test do
  use ExUnit.Case
  import AdventOfCode.Day08

  test "combinations: k = 3" do
    assert combinations(3, [1,2,3,4,5]) === [
      [1, 2, 3],
      [1, 2, 4],
      [1, 2, 5],
      [1, 3, 4],
      [1, 3, 5],
      [1, 4, 5],
      [2, 3, 4],
      [2, 3, 5],
      [2, 4, 5],
      [3, 4, 5],
    ]
  end

  test "combinations: k = 2" do
    assert combinations(2, [1,2,3,4,5]) === [
      [1, 2],
      [1, 3],
      [1, 4],
      [1, 5],
      [2, 3],
      [2, 4],
      [2, 5],
      [3, 4],
      [3, 5],
      [4, 5],
    ]
  end

  test "antinodes" do
    assert antinodes({2, 4}, {4, 5}) === [{0, 3}, {6, 6}]
    assert antinodes({3, 5}, {5, 4}) === [{1, 6}, {7, 3}]
    assert antinodes({4, 5}, {2, 4}) === [{6, 6}, {0, 3}]
    assert antinodes({4, 4}, {2, 5}) === [{6, 3}, {0, 6}]
  end

  test "part 1" do
    input = """
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    """

    assert part1(input) === 14
  end

  test "part 2" do
    input = """
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    """

    assert part2(input) === 34
  end
  
  test "antinodes 2 sub" do
    assert antinodes_2_sub({2, 4}, {1, 2}, 10, 10) === [{2, 4}, {1, 2}, {0, 0}]
  end

  test "antinodes 2 add" do
    assert antinodes_2_add({2, 4}, {1, 2}, 10, 10) === [{2, 4}, {3, 6}, {4, 8}]
  end  
end
