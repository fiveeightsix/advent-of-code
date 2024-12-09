defmodule AdventOfCode.Day07Test do
  use ExUnit.Case
  import AdventOfCode.Day07

  test "parse equation" do
    line = "190: 10 19"

    assert {190, [10, 19]} === parse_equation(line)
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

  test "part 2" do
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

    assert part2(input) === 11387
  end
end

defmodule AdventOfCode.Day07Part1Test do
  use ExUnit.Case
  import AdventOfCode.Day07.Part1

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
end

defmodule AdventOfCode.Day07Part2Test do
  use ExUnit.Case
  import AdventOfCode.Day07.Part2

  test "concatenate" do
    assert 12 === concatenate("1", "2")
  end

  test "find operators 2 '190: 10 19'" do
    total = 190
    terms = [10, 19]

    assert {:ok, [:mul], 190} === find_operators_2({total, terms})
  end

  test "find operators 2 '292: 11 6 16 20'" do
    total = 292
    terms = [11, 6, 16, 20]

    assert {:ok, [:add, :mul, :add], 292} === find_operators_2({total, terms})
  end

  test "find operators 2 '3267: 81 40 27' (multiple answers)" do
    total = 3267
    terms = [81, 40, 27]

    {result, _, _} = find_operators_2({total, terms})

    assert result === :ok
  end

  test "find operators 2 '156: 15 6'" do
    total = 156
    terms = [15, 6]

    assert {:ok, [:con], 156} === find_operators_2({total, terms})
  end

  test "find operators 2 '7290: 6 8 6 15'" do
    total = 7290
    terms = [6, 8, 6, 15]

    assert {:ok, [:mul, :con, :mul], 7290} === find_operators_2({total, terms})
  end
end
