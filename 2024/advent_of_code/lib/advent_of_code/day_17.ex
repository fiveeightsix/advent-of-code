defmodule AdventOfCode.Day17 do
  defmodule Part1Parser.Helpers do
    import NimbleParsec

    def to_2_tuple(parsed_values) do
      [first, second | _] = parsed_values
      {first, second}
    end
    
    def register(name, label) do
      ignore(string("Register "))
      |> ignore(string(name))
      |> ignore(string(": "))
      |> integer(min: 1)
      |> unwrap_and_tag(label)
    end

    def registers() do
      eventually(register("A", :a))
      |> eventually(register("B", :b))
      |> eventually(register("C", :c))
      |> reduce({Map, :new, []})
      |> unwrap_and_tag(:registers)
    end
    
    def instruction() do
      integer(min: 1, max: 2)
      |> optional(ignore(string(",")))
    end
    
    def program() do
      ignore(string("Program: "))
      |> repeat(instruction())
      |> tag(:program)
    end
  end

  defmodule Part1Parser do
    import NimbleParsec
    import Part1Parser.Helpers

    computer =  eventually(registers()) |> eventually(program())
    
    defparsec(:computer, computer)
  end

  def get_combo_value(registers, operand) do
    case operand do
      0 -> 0
      1 -> 1
      2 -> 2
      3 -> 3
      4 -> registers.a
      5 -> registers.b
      6 -> registers.c
    end
  end

  @doc """
  Performs division. The numerator is the value in the A register. The denominator is
  found by raising 2 to the power of the instruction's combo operand. The result of
  the division operation is truncated to an integer and then written to the A register.
  """
  def instruction_adv(registers, operand) do
    numerator = registers.a
    denominator = Integer.pow(2, get_combo_value(registers, operand))
    value = Integer.floor_div(numerator, denominator)

    {%{registers | a: value}, :no_output, :no_jump}
  end

  @doc """
  Calculates the bitwise XOR of register B and the instruction's literal operand,
  then stores the result in register B.
  """
  def instruction_bxl(registers, operand) do
    value = Bitwise.bxor(registers.b, operand)

    {%{registers | b: value}, :no_output, :no_jump}
  end

  @doc """
  Calculates the value of its combo operand modulo 8 (thereby keeping only its lowest
  3 bits), then writes that value to the B register.
  """
  def instruction_bst(registers, operand) do
    value = registers |> get_combo_value(operand) |> rem(8)
    
    {%{registers | b: value}, :no_output, :no_jump}
  end

  @doc """
  Does nothing if the A register is 0. However, if the A register is not zero, it jumps
  by setting the instruction pointer to the value of its literal operand.
  """
  def instruction_jnz(registers, operand) do
    case registers.a do
      0 -> {registers, :no_output, :no_jump}
      _ -> {registers, :no_output, operand}
    end
  end

  @doc """
  Calculates the bitwise XOR of register B and register C, then stores the result in
  register B. (For legacy reasons, this instruction reads an operand but ignores it.)
  """
  def instruction_bxc(registers, _) do
    value = Bitwise.bxor(registers.b, registers.c)

    {%{registers | b: value}, :no_output, :no_jump}
  end

  @doc """
  Calculates the value of its combo operand modulo 8, then outputs that value.
  """
  def instruction_out(registers, operand) do
    output = registers |> get_combo_value(operand) |> rem(8)

    {registers, output, :no_jump}
  end

  @doc """
  Works exactly like the adv instruction except that the result is stored in the B
  register. (The numerator is still read from the A register.)
  """
  def instruction_bdv(registers, operand) do
    numerator = registers.a
    denominator = Integer.pow(2, get_combo_value(registers, operand))
    value = Integer.floor_div(numerator, denominator)

    {%{registers | b: value}, :no_output, :no_jump}
  end

  @doc """
  Works exactly like the adv instruction except that the result is stored in the C
  register. (The numerator is still read from the A register.)
  """
  def instruction_cdv(registers, operand) do
    numerator = registers.a
    denominator = Integer.pow(2, get_combo_value(registers, operand))
    value = Integer.floor_div(numerator, denominator)

    {%{registers | c: value}, :no_output, :no_jump}
  end

  def run(program_map, pointer, _) when pointer >= map_size(program_map), do: []
  
  def run(program_map, pointer, registers) do
    opcode = Map.get(program_map, pointer)
    operand = Map.get(program_map, pointer + 1)

    {updated_registers, output, jump} =
      case opcode do
        0 -> instruction_adv(registers, operand)
        1 -> instruction_bxl(registers, operand)
        2 -> instruction_bst(registers, operand)
        3 -> instruction_jnz(registers, operand)
        4 -> instruction_bxc(registers, operand)
        5 -> instruction_out(registers, operand)
        6 -> instruction_bdv(registers, operand)
        7 -> instruction_cdv(registers, operand)
      end

    next_pointer =
      case jump do
        :no_jump -> pointer + 2
        jump -> jump
      end

    case output do
      :no_output -> run(program_map, next_pointer, updated_registers)
      output -> [output] ++ run(program_map, next_pointer, updated_registers)
    end
  end
  
  def part1(input) do
    {_, result, _, _, _, _} = Part1Parser.computer(input)

    [registers: registers, program: program] = result

    program_map =
      program
      |> Enum.with_index(fn number, index -> {index, number} end)
      |> Map.new()

    run(program_map, 0, registers)
    |> Enum.join(",")
  end
end
