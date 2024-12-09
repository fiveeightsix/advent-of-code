defmodule AdventOfCode.Day08 do
  def make_map(input) do
    map =
      input
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

  def combinations(0, _), do: [[]]
  def combinations(_, []), do: []

  def combinations(k, [head | tail]) do
    for(l <- combinations(k - 1, tail), do: [head | l]) ++ combinations(k, tail)
  end

  def antinodes({ax, ay}, {bx, by}) do
    {dx, dy} = {bx - ax, by - ay}

    [{ax - dx, ay - dy}, {bx + dx, by + dy}]
  end

  def outside_map?({x, y}, width, height) do
    x < 0 or x >= width or y < 0 or y >= height
  end

  def part1(input) do
    {map, width, height} = make_map(input)

    locate_antennas(map)
    |> Enum.map(fn {_, v} -> combinations(2, v) end)
    |> Enum.flat_map(fn antenna_pair -> antenna_pair end)
    |> Enum.flat_map(fn [a, b] -> antinodes(a, b) end)
    |> Enum.reject(fn antinode -> outside_map?(antinode, width, height) end)
    |> Enum.uniq()
    |> Enum.count()
  end

  defguard is_inside_map(x, y, width, height) when x >= 0 and x < width and y >= 0 and y < height

  def antinodes_2_add({x, y} = a, {dx, dy} = d, width, height)
      when is_inside_map(x, y, width, height) do
    [a | antinodes_2_add({x + dx, y + dy}, d, width, height)]
  end

  def antinodes_2_add(_, _, _, _), do: []

  def antinodes_2_sub({x, y} = a, {dx, dy} = d, width, height)
      when is_inside_map(x, y, width, height) do
    [a | antinodes_2_sub({x - dx, y - dy}, d, width, height)]
  end

  def antinodes_2_sub(_, _, _, _), do: []

  def antinodes_2({ax, ay} = a, {bx, by} = b, width, height) do
    d = {bx - ax, by - ay}

    antinodes_2_sub(a, d, width, height) ++ antinodes_2_add(b, d, width, height)
  end

  def part2(input) do
    {map, width, height} = make_map(input)

    locate_antennas(map)
    |> Enum.map(fn {_, v} -> combinations(2, v) end)
    |> Enum.flat_map(fn antenna_pair -> antenna_pair end)
    |> Enum.flat_map(fn [a, b] -> antinodes_2(a, b, width, height) end)
    |> Enum.uniq()
    |> Enum.count()
  end
end
