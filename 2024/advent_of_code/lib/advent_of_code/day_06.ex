defmodule AdventOfCode.Day06 do
  def find_in_column(row, x \\ 0)
  
  def find_in_column([head | tail], x) do
    case head do
      "^" -> {:some, x}
      _ -> find_in_column(tail, x + 1)
    end
  end

  def find_in_column([], _) do
    {:none}
  end

  def find_in_row(map, y \\ 0)

  def find_in_row([head | tail], y) do
    case find_in_column(head) do
      {:some, x} -> {:some, x, y}
      _ -> find_in_row(tail, y + 1)
    end
  end

  def find_in_row([], _) do
    {:none}
  end
  
  def make_map(input) do
    map = input
    |> String.split("\n", trim: true)
    |> Enum.map(fn x -> String.split(x, "", trim: true) end)

    height = Enum.count(map)
    width = Enum.count(List.first(map))

    {map, width, height}
  end

  def in_front({x, y}, direction) do
    case direction do
      :north -> {x, y - 1}
      :east -> {x + 1, y}
      :south -> {x, y + 1}
      :west -> {x - 1, y}
    end
  end

  def turn_right(direction) do
    case direction do
      :north -> :east
      :east -> :south
      :south -> :west
      :west -> :north
    end
  end

  def step({map, width, height}, current_position, direction, path) do
    case in_front(current_position, direction) do
      {x, _} when x < 0 -> path
      {x, _} when x >= width -> path
      {_, y} when y < 0 -> path
      {_, y} when y >= height -> path
      {x, y} ->
        case Enum.at(Enum.at(map, y), x) do
          "#" -> step({map, width, height}, current_position, turn_right(direction), path)
          _ -> step({map, width, height}, {x, y}, direction, [{x, y} | path])
        end
    end
  end
  
  def part1(input) do
    {map, width, height} = make_map(input)

    {_, x, y} = find_in_row(map)

    step({map, width, height}, {x, y}, :north, [])
    |> Enum.uniq
    |> Enum.count
  end
end
