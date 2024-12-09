defmodule Mix.Tasks.D09.P1 do
  use Mix.Task

  import AdventOfCode.Day09

  @shortdoc "Day 09 part 1"
  def run(args) do
    path = Path.join(File.cwd!, "lib/input/day_09.txt")

    case File.read(path) do
      {:ok, input} ->
        if Enum.member?(args, "-b") do
          Benchee.run(%{part_1: fn input -> input |> part1() end})
        else
          input
          |> part1()
          |> IO.inspect(label: "Part 1 results")
        end
      {:error, message} -> IO.puts(message)
    end
  end
end