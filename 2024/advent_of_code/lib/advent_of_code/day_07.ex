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
  
  defp find_operators_rec({total, []}, operators, current) when current === total do
    {:ok, operators, current}
  end

  defp find_operators_rec({total, [head | tail]}, operators, current) when current <= total do
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

  defp find_operators_rec({_, []}, _, _), do: {:fail}
  defp find_operators_rec({_, [_ | _]}, _, _), do: {:fail}
  
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> parse_equation()
      |> find_operators()
    end)
    |> Enum.map(fn x ->
      case x do
        {:ok, _, total} -> total
        {:fail} -> 0
      end
    end)
    |> Enum.sum()
  end
  
  def concatenate(a, b), do: Enum.join([a, b], "") |> String.to_integer()
  
  def find_operators_2({total, terms}) do
    [first_term | remaining_terms] = terms
    find_operators_2_rec({total, remaining_terms}, [], first_term)
  end 
  
  defp find_operators_2_rec({total, []}, operators, current) when current === total do
    {:ok, operators, current}
  end

  defp find_operators_2_rec({total, [head | tail]}, operators, current) when current <= total do
    lr = {
      find_operators_2_rec({total, tail}, [:add | operators], current + head),
      find_operators_2_rec({total, tail}, [:mul | operators], current * head),
      find_operators_2_rec({total, tail}, [:con | operators], concatenate(current, head))
    }
    
    case lr do
      {{:fail}, {:fail}, {:fail}} -> {:fail}
      {{:ok, operators, curent}, {:fail}, {:fail}} -> {:ok, operators, curent}
      {_, {:ok, operators, curent}, {:fail}} -> {:ok, operators, curent}
      {_, _, {:ok, operators, curent}} -> {:ok, operators, curent}
    end
  end

  defp find_operators_2_rec({_, []}, _, _), do: {:fail}
  defp find_operators_2_rec({_, [_ | _]}, _, _), do: {:fail}

  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> parse_equation()
      |> find_operators_2()
    end)
    |> Enum.map(fn x ->
      case x do
        {:ok, _, total} -> total
        {:fail} -> 0
      end
    end)
    |> Enum.sum()
  end
end
