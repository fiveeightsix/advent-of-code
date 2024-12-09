defmodule AdventOfCode.Day09 do
  def disk_layout(input) do
    layout =
      input
      |> String.split("", trim: true)
      |> Enum.map(&Integer.parse/1)
      |> Enum.reject(fn x -> x === :error end)
      |> Enum.map(fn {i, _} -> i end)
      |> Enum.with_index()
      |> Enum.flat_map(fn {element, index} ->
        case {element, index} do
          {0, _} ->
            []

          {element, index} when rem(index, 2) === 0 ->
            for _ <- 1..element, do: {:file, div(index, 2)}

          {element, _} ->
            for _ <- 1..element, do: {:space}
        end
      end)
      |> Enum.with_index()
      |> Map.new(fn {element, index} -> {index, element} end)

    {layout, map_size(layout)}
  end

  def compact(layout, a, b) when a >= b, do: layout

  def compact(layout, a, b) do
    case {layout[a], layout[b]} do
      {{:space}, {:space}} -> compact(layout, a, b - 1)
      {{:space}, {:file, _}} -> compact(%{layout | a => layout[b], b => layout[a]}, a + 1, b - 1)
      {{:file, _}, _} -> compact(layout, a + 1, b)
    end
  end

  def part1(input) do
    {layout, size} = disk_layout(input)

    compacted_layout = compact(layout, 0, size - 1)

    0..(size - 1)
    |> Enum.map(fn i -> {compacted_layout[i], i} end)
    |> Enum.reduce(0, fn {block, i}, checksum ->
      case block do
        {:file, file_id} -> checksum + file_id * i
        {:space} -> checksum
      end
    end)
  end

  def layout_to_string({layout, size}) do
    0..(size - 1)
    |> Enum.map(fn i -> layout[i] end)
    |> Enum.map(fn x ->
      case x do
        {:file, file_id} -> Integer.to_string(file_id)
        {:space} -> "."
      end
    end)
    |> Enum.join()
  end
end
