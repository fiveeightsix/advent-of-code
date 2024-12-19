defmodule AdventOfCode.Day17 do
  defmodule Part1Parser.Helpers do
    import NimbleParsec

    def to_register(parsed_values) do
      [name, value | _] = parsed_values
      {name, value}
    end
    
    def register(name) do
      ignore(string("Register "))
      |> string(name)
      |> ignore(string(": "))
      |> integer(min: 1)
      |> reduce({:to_register, []})
    end

    def registers() do
      eventually(register("A"))
      |> eventually(register("B"))
      |> eventually(register("C"))      
      |> tag(:registers)
    end

    def to_instruction(parsed_values) do
      [opcode, operand | _] = parsed_values
      {opcode, operand}
    end
    
    def instruction() do
      integer(min: 1, max: 2)
      |> ignore(string(","))
      |> integer(min: 1, max: 2)
      |> optional(ignore(string(",")))
      |> reduce({:to_instruction, []})
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

  def part1(input) do
    Part1Parser.computer(input)
  end
end
