defmodule AdventOfCode.Day09Test do
  use ExUnit.Case
  import AdventOfCode.Day09

  test "disk layout" do
    input = """
    2333133121414131402
    """

    expected = "00...111...2...333.44.5555.6666.777.888899"

    actual = input |> disk_layout() |> layout_to_string()

    assert actual === expected
  end

  test "compact" do
    input = """
    2333133121414131402
    """

    expected = "0099811188827773336446555566.............."

    {layout, size} = disk_layout(input)

    compacted_layout = compact(layout, 0, size - 1)

    actual = layout_to_string({compacted_layout, size})

    assert actual === expected
  end

  test "part 1" do
    input = """
    2333133121414131402
    """

    assert part1(input) === 1928
  end
end
