defmodule AdventOfCode.Day03 do
  defmodule Part1Parser do
    import NimbleParsec

    mul =
      ignore(string("mul("))
      |> integer(min: 1, max: 3)
      |> ignore(string(","))
      |> integer(min: 1, max: 3)
      |> ignore(string(")"))

    defparsec :mul, mul |> eventually |> repeat
  end

  def multiply_elements(list) do
    Enum.flat_map_reduce(list, 1, fn y, acc -> {[y], y * acc} end) |> elem(1)
  end
  
  def part1(input) do
    case Part1Parser.mul(input) do
      {:ok, results, _, _, _, _} ->
        results
        |> Enum.chunk_every(2)
        |> Enum.map(&multiply_elements/1)
        |> Enum.sum
      {:error, message} -> message
    end
  end
end
