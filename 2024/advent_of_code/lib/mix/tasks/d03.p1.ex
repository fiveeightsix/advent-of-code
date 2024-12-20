defmodule Mix.Tasks.D03.P1 do
  use Mix.Task

  import AdventOfCode.Day03

  @shortdoc "Day 03 Part 1"
  def run(_args) do
    path = Path.join(File.cwd!(), "lib/input/day_03.txt")

    case File.read(path) do
      {:ok, input} ->
        input
        |> part1
        |> IO.inspect(label: "Part 1 results")

      {:error, message} ->
        IO.puts(message)
    end
  end
end
