defmodule Mix.Tasks.D17.P1 do
  use Mix.Task

  import AdventOfCode.Day17

  @shortdoc "Day 17 part 1"
  def run(args) do
    path = File.cwd!() |> Path.join("lib/input/day_17.txt")

    case File.read(path) do
      {:ok, input} ->
        if Enum.member?(args, "-b") do
          Benchee.run(%{part_1: fn -> input |> part1() end})
        else
          input
          |> part1()
          |> IO.inspect(label: "Part 1 results")
        end

      {:error, message} ->
        IO.puts(message)
    end
  end
end
