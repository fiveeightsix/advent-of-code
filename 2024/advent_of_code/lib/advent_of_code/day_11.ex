defmodule AdventOfCode.Day11 do
  def make_plot_map(input) do
    {plot_map, max_x, x, y} =
      input
      |> String.split("", trim: true)
      |> Enum.reduce({Map.new(), 0, 0, 0}, fn c, {plot_map, max_x, x, y} ->
        case c do
          "\n" -> {plot_map, max_x, 0, y + 1}
          _ -> {Map.put(plot_map, {x, y}, c), max(max_x, x + 1), x + 1, y}
        end
      end)

    {plot_map, max_x, y}
  end

  def get_neighbour_plots(plot_map, {x, y}) do
    [{x, y - 1}, {x + 1, y}, {x, y + 1}, {x - 1, y}]
    |> Enum.map(fn point -> {point, Map.get(plot_map, point)} end)
  end

  def find_region(plot_map, unvisited, [], found), do: {unvisited, [], found}

  def find_region(plot_map, unvisited, queued_plots, found) do
    [{point, plant} | remaining_plots] = queued_plots

    neighbour_plots = plot_map |> get_neighbour_plots(point)

    similar_neighbour_plots = neighbour_plots |> Enum.reject(fn {_, v} -> v !== plant end)

    outside_edges = Enum.count(neighbour_plots) - Enum.count(similar_neighbour_plots)

    next_plots =
      similar_neighbour_plots
      |> Enum.reject(fn {point, _} -> MapSet.member?(unvisited, point) === false end)

    next_points = next_plots |> Enum.map(fn {point, _} -> point end)

    find_region(
      plot_map,
      MapSet.difference(unvisited, MapSet.new([point | next_points])),
      next_plots ++ remaining_plots,
      [{point, outside_edges} | found]
    )
  end

  def group_plots(plot_map, unvisited) do
    if MapSet.size(unvisited) === 0 do
      []
    else
      point = Enum.random(unvisited)
      plant = Map.get(plot_map, point)

      {updated_unvisited, _, found} = find_region(plot_map, unvisited, [{point, plant}], [])

      [{plant, found} | group_plots(plot_map, updated_unvisited)]
    end
  end

  def part1(input) do
    {plot_map, _, _} = make_plot_map(input)

    plot_map
    |> group_plots(MapSet.new(Map.keys(plot_map)))
    |> Enum.map(fn {_, points} ->
      region_perimiter =
        points
        |> Enum.map(fn {_, outside_edges} -> outside_edges end)
        |> Enum.sum()

      region_area = Enum.count(points)

      region_perimiter * region_area
    end)
    |> Enum.sum()
  end
end
