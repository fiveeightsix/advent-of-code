defmodule AdventOfCode.Day08 do
  def make_map(input) do
    map = input
    |> String.split("\n", trim: true)
    |> Enum.map(fn x -> String.split(x, "", trim: true) end)

    height = Enum.count(map)
    width = Enum.count(List.first(map))

    {map, width, height}
  end
  
  def locate_antennas(map) do
    map
    |> Enum.with_index(fn row, y -> {y, row} end)
    |> Enum.reduce(Map.new(), fn {y, row}, locations ->
      row
      |> Enum.with_index(fn cell, x -> {x, cell} end)
      |> Enum.reduce(locations, fn {x, cell}, locations ->
        if cell !== "." do
          Map.update(locations, cell, [{x, y}], fn current_value ->
            [{x, y} | current_value]
          end)
        else
          locations
        end
      end)
    end)
  end

  def antinodes({ax, ay}, {bx, by}) do
    {dx, dy} = {bx - ax, by - ay}
    
    [{ax - dx, ay - dy}, {bx + dx, by + dy}]
  end

  def combinations(0, _), do: [[]]
  def combinations(_, []), do: []
  def combinations(k, [head|tail]) do
    (for l <- combinations(k - 1, tail), do: [head | l]) ++ combinations(k, tail)
  end

  def is_outside_map({x, y}, width, height) do
    x < 0 or x >= width or y < 0 or y >= height
  end
  
  def part1(input) do
    {map, width, height} = make_map(input)

    locate_antennas(map)
    |> Enum.map(fn {_, v} -> combinations(2, v) end)
    |> Enum.flat_map(fn antenna_pair -> antenna_pair end)
    |> Enum.flat_map(fn [a, b] -> antinodes(a, b) end)
    |> Enum.reject(fn antinode -> is_outside_map(antinode, width, height) end)
    |> Enum.uniq()
    |> Enum.count()
  end
end
