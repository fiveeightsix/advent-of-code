defmodule AdventOfCode.Day07Test do
  use ExUnit.Case

  import AdventOfCode.Day07

  test "parse equation" do
    line = "190: 10 19"

    assert { 190, [10, 19] } === parse_equation(line)
  end

  test "find operators '190: 10 19'" do
    total = 190
    terms = [10, 19]

    assert {:ok, [:mul], 190} === find_operators({total, terms})
  end
  
  test "find operators '292: 11 6 16 20'" do
    total = 292
    terms = [11, 6, 16, 20]

    assert {:ok, [:add, :mul, :add], 292} === find_operators({total, terms})
  end

  test "find operators '3267: 81 40 27' (multiple answers)" do
    total = 3267
    terms = [81, 40, 27]

    {result, _, _} = find_operators({total, terms})
    
    assert result === :ok
  end

  test "part 1" do
    input = """
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    """

    assert part1(input) === 3749
  end
end

    
