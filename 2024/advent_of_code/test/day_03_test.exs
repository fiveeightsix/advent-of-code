defmodule AdventOfCode.Day03Test do
  use ExUnit.Case

  import AdventOfCode.Day03

  test "multiply_elements" do
    assert multiply_elements([0]) == 0
    assert multiply_elements([1]) == 1
    assert multiply_elements([1, 1]) == 1
    assert multiply_elements([2, 5]) == 10
  end

  test "part 1" do
    assert part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))") == 161
  end
end
