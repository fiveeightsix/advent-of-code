defmodule AdventOfCode.Day06Test do
  use ExUnit.Case

  import AdventOfCode.Day06
  test "make_map" do
    input = """
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    """
    
    {_, width, height} = make_map(input)

    assert width == 10
    assert height == 10
  end

  test "find in column - not found" do
    assert find_in_column(String.split("....#.....", "", trim: true)) === {:none}
  end

  test "find in column - found" do
    assert find_in_column(String.split(".#..^.....", "", trim: true)) === {:some, 4}
  end

  test "find in row - not found" do
    input = """
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#........
    ........#.
    #.........
    ......#...
    """

    {map, _, _} = make_map(input)

    assert find_in_row(map) === {:none}
  end

  test "find in row - found" do
    input = """
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    """

    {map, _, _} = make_map(input)

    assert find_in_row(map) === {:some, 4, 6}
  end

  test "in_front" do
    assert in_front({0, 0}, :north) === {0, -1}
    assert in_front({0, 0}, :east) === {1, 0}
    assert in_front({0, 0}, :south) === {0, 1}
    assert in_front({0, 0}, :west) === {-1, 0}
  end
  
  test "step - at boundary north" do
    map = [[".", ".", "."], [".", ".", "."], [".", ".", "."]]

    expected_path =  [{1, 1}]
    
    assert step({map, 3, 3}, {1, 0}, :north, expected_path) === expected_path
  end

  test "step - at boundary east" do
    map = [[".", ".", "."], [".", ".", "."], [".", ".", "."]]

    expected_path =  [{1, 1}]
    
    assert step({map, 3, 3}, {2, 1}, :east, expected_path) === expected_path
  end

  test "step - at boundary south" do
    map = [[".", ".", "."], [".", ".", "."], [".", ".", "."]]

    expected_path =  [{1, 1}]
    
    assert step({map, 3, 3}, {1, 2}, :south, expected_path) === expected_path
  end

  test "step - at boundary west" do
    map = [[".", ".", "."], [".", ".", "."], [".", ".", "."]]

    expected_path =  [{1, 1}]
    
    assert step({map, 3, 3}, {0, 1}, :west, expected_path) === expected_path
  end


  test "part 1" do
    input = """
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    """

    assert part1(input) == 41
  end
end

    
