defmodule Mix.Tasks.D08.P1 do
  use Mix.Task

  import AdventOfCode.Day08

  @shortdoc "Day 08 part 1"
  def run(_args) do
    path = Path.join(File.cwd!, "lib/input/day_08.txt")

    case File.read(path) do
      {:ok, input} -> input
          |> part1
          |> IO.inspect(label: "Part 1 results")
      {:error, message} -> IO.puts(message)
    end
  end
end
