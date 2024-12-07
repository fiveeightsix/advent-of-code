defmodule AdventOfCode.Day07 do
  def parse_equation(line) do
    [total, rhs] = String.split(line, ": ", trim: true)
    
    terms = String.split(rhs, " ", trim: true)
    |> Enum.map(&String.to_integer/1)
    
    {String.to_integer(total), terms}
  end

  def find_operators({total, terms}) do
    [first_term | remaining_terms] = terms
    find_operators_rec({total, remaining_terms}, [], first_term)
  end
 
  
  def find_operators_rec({total, []}, operators, current) when current === total do
    {:ok, operators, current}
  end

  def find_operators_rec({total, []}, operators, current) do
    {:fail}
  end

  def find_operators_rec({total, [head | tail]}, operators, current) when current <= total do
    lr = {
      find_operators_rec({total, tail}, [:add | operators], current + head),
      find_operators_rec({total, tail}, [:mul | operators], current * head),
    }

    case lr do
      {{:fail}, {:fail}} -> {:fail}
      {{:ok, operators, curent}, {:fail}} -> {:ok, operators, curent}
      {_, {:ok, operators, curent}} -> {:ok, operators, curent}
    end
  end

  def find_operators_rec({total, [head | _]}, operators, current) do
    {:fail}
  end
  
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> parse_equation
      |> find_operators
    end)
    |> Enum.map(fn x ->
      case x do
        {:ok, _, total} -> total
        {:fail} -> 0
      end
    end)
    |> Enum.sum
  end
end
