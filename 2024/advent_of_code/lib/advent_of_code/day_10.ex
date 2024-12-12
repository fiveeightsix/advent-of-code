defmodule AdventOfCode.Day10 do
  def make_map(input) do
    {map, trailheads, max_x, x, y} =
      input
      |> String.split("", trim: true)
      |> Enum.reduce({Map.new(), [], 0, 0, 0}, fn c, {map, trailheads, max_x, x, y} ->
        case Integer.parse(c) do
          {height, _} when height === 0 ->
            {Map.put(map, {x, y}, height), [{x, y} | trailheads], max(max_x, x + 1), x + 1, y}

          {height, _} ->
            {Map.put(map, {x, y}, height), trailheads, max(max_x, x + 1), x + 1, y}

          :error ->
            {map, trailheads, max_x, 0, y + 1}
        end
      end)

    {map, trailheads, max_x, y}
  end

  def get_neighbours(map, {x, y}) do
    [{x, y - 1}, {x + 1, y}, {x, y + 1}, {x - 1, y}]
    |> Enum.map(fn point -> {point, Map.get(map, point)} end)
    |> Enum.reject(fn {point, height} -> height === nil end)
  end

  def climb(point, map), do: climb_rec(map, MapSet.new(), [{point, 0}], 0)

  def climb_rec(_, _, [], score), do: score

  def climb_rec(map, visited, queue, score) do
    [{point, height} | remaining] = queue

    new_points =
      map
      |> get_neighbours(point)
      |> Enum.reject(fn {neighbour_point, _} ->
        MapSet.member?(visited, neighbour_point) === true
      end)
      |> Enum.reject(fn {_, neighbour_height} -> neighbour_height !== height + 1 end)

    climb_rec(
      map,
      MapSet.put(visited, point),
      new_points ++ remaining,
      case height do
        9 -> score + 1
        _ -> score
      end
    )
  end

  def part1(input) do
    {map, trailheads, width, height} = make_map(input)

    trailheads
    |> Enum.map(fn trailhead -> climb(trailhead, map) end)
    |> Enum.sum()
  end
end
