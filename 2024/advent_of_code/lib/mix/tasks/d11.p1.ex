defmodule Mix.Tasks.D11.P1 do
  use Mix.Task

  import AdventOfCode.Day11

  @shortdoc "Day 11 part 1"
  def run(args) do
    path = File.cwd!() |> Path.join("lib/input/day_11.txt")

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
