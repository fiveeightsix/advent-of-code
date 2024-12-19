defmodule AdventOfCode.Day17 do
  defmodule Part1Parser.Helpers do
    import NimbleParsec

    def to_2_tuple(parsed_values) do
      [first, second | _] = parsed_values
      {first, second}
    end
    
    def register(name) do
      ignore(string("Register "))
      |> string(name)
      |> ignore(string(": "))
      |> integer(min: 1)
      |> reduce({:to_2_tuple, []})
    end

    def registers() do
      eventually(register("A"))
      |> eventually(register("B"))
      |> eventually(register("C"))
      |> reduce({Map, :new, []})
      |> unwrap_and_tag(:registers)
    end
    
    def instruction() do
      integer(min: 1, max: 2)
      |> ignore(string(","))
      |> integer(min: 1, max: 2)
      |> optional(ignore(string(",")))
      |> reduce({:to_2_tuple, []})
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
      4 -> Map.get(registers, "A")
      5 -> Map.get(registers, "B")
      6 -> Map.get(registers, "C")
    end
  end

  def instruction_adv(registers, operand) do

  end

  def instruction_bxl(registers, operand) do
    numerator = Map.get(registers, "A")
    denominator = get_combo_value(registers, operand) ** 2
    numerator / denominator 
  end

  def instruction_bst(registers, operand) do
    value = registers |> get_combo_value(operand) |> rem(8)
    
    %{registers | "B" => value}
  end
  
  def instruction_jnz(registers, operand) do

  end

  def instruction_bxc(registers, operand) do

  end

  def instruction_out(registers, operand) do

  end

  def instruction_bdv(registers, operand) do

  end

  def instruction_cdv(registers, operand) do

  end
  
  # def run(program, pointer, registers, output) do

  #   updated_registers = case     
  # end
  
  def part1(input) do
    {_, result, _, _, _, _} = Part1Parser.computer(input)

    result
  end
end
