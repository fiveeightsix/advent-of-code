defmodule AdventOfCode.Day17Test do
  use ExUnit.Case
  import AdventOfCode.Day17

  test "opcode 1: instruction bxl" do
    registers = %{a: 0, b: 29, c: 0}
    {actual_registers, _, _} = instruction_bxl(registers, 7)

    assert actual_registers === %{a: 0, b: 26, c: 0}
  end
  
  test "opcode 2: instruction bst" do
    registers = %{a: 0, b: 0, c: 9}
    {actual_registers, _, _} = instruction_bst(registers, 6)

    assert actual_registers === %{a: 0, b: 1, c: 9}
  end

  test "opcode 4: instruction bxc" do
    registers = %{a: 0, b: 2024, c: 43690}
    {actual_registers, _, _} = instruction_bxc(registers, 0)

    assert actual_registers === %{a: 0, b: 44354, c: 43690}
  end

  test "opcode 3: instruction jnz - does nothing" do
    registers = %{a: 0, b: 0, c: 0}
    operand = 8
    {_, _, jump} = instruction_jnz(registers, operand)

    assert jump === :no_jump
  end

  test "opcode 3: instruction jnz - jump to operand" do
    registers = %{a: 1, b: 0, c: 0}
    operand = 8
    {_, _, jump} = instruction_jnz(registers, operand)

    assert jump === operand
  end
  
  test "part 1: A: 10, Program: 5,0,5,1,5,4" do
    input = """
    Register A: 10
    Register B: 0
    Register C: 0

    Program: 5,0,5,1,5,4
    """

    assert part1(input) === "0,1,2"
  end

  test "part 1: A: 2024, the program 0,1,5,4,3,0" do
    input = """
    Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    """

    assert part1(input) === "4,2,5,6,7,7,7,7,3,1,0"
  end
    
  test "part 1" do
    input = """
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    """

    assert part1(input) === "4,6,3,5,6,3,5,2,1,0"
  end
end
