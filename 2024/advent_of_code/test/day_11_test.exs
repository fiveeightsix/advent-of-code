defmodule AdventOfCode.Day11Test do
  use ExUnit.Case
  import AdventOfCode.Day11

  test "find region" do
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

    {plot_map, _, _} = make_plot_map(input)

    {unvisited, _, found} =
      find_region(plot_map, MapSet.new(Map.keys(plot_map)), [{{0, 0}, "R"}], [])

    assert Enum.count(found) === 12

    assert map_size(plot_map) - MapSet.size(unvisited) === 12
  end

  test "part 1 - small sample" do
    input = """
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO
    """

    assert part1(input) === 772
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
