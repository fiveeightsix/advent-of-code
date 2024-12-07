defmodule Mix.Tasks.D07.P1 do
  use Mix.Task

  import AdventOfCode.Day07

  @shortdoc "Day 07 part 1"
  def run(_args) do
    path = Path.join(File.cwd!, "lib/input/day_07.txt")

    case File.read(path) do
      {:ok, input} -> input
          |> part1
          |> IO.inspect(label: "Part 1 results")
      {:error, message} -> IO.puts(message)
    end
  end
end
