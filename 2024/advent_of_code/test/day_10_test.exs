defmodule AdventOfCode.Day10Test do
  use ExUnit.Case
  import AdventOfCode.Day10

  test "Make map - finds trail heads" do
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

    expected_trailheads = [{2, 0}, {4, 0}, {4, 2}, {6, 4}, {2, 5}, {5, 5}, {0, 6}, {6, 6}, {1, 7}]

    {_, actual_trailheads, width, height} = make_map(input)

    assert Enum.sort(actual_trailheads) === Enum.sort(expected_trailheads)
    assert width === 8
    assert height === 8
  end

  test "Climb" do
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

    {map, _, _, _} = make_map(input)

    assert climb({2, 0}, map) === 5
  end

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
