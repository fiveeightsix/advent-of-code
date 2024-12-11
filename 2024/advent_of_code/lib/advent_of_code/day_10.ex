defmodule AdventOfCode.Day10 do
  def make_map(input) do
    parsed_input =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn x ->
        x
        |> String.split("", trim: true)
        |> Enum.map(&Integer.parse/1)
        |> Enum.reject(fn x -> x === :error end)
        |> Enum.map(fn {x, _} -> x end)
      end)

    max_x = Enum.count(parsed_input)
    max_y = Enum.count(List.first(parsed_input))

    map =
      parsed_input
      |> Enum.with_index(fn row, y -> {y, row} end)
      |> Enum.reduce(Map.new(), fn {y, row}, map ->
        row
        |> Enum.with_index(fn height, x -> {x, height} end)
        |> Enum.reduce(map, fn {x, height}, map ->
          Map.put(map, {x, y}, height)
        end)
      end)
      
    {map, max_x, max_y}
  end
  
  def part1(input) do
    {map, max_x, max_y} = make_map(input)
  end
end
