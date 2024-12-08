defmodule AdventOfCode.Day08 do
  def make_map(input) do
    map = input
    |> String.split("\n", trim: true)
    |> Enum.map(fn x -> String.split(x, "", trim: true) end)

    height = Enum.count(map)
    width = Enum.count(List.first(map))

    {map, width, height}
  end

  def locate_antennas_row(row) do
    row

  end     
  
  def locate_antennas(map) do
    map
    |> Enum.with_index(fn row, y -> {y, row} end)
    |> Enum.reduce(%{}, fn {y, row}, locations ->
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
  
  def part1(input) do
    {map, _width, _height} = make_map(input)

    locate_antennas(map)
  end
end
