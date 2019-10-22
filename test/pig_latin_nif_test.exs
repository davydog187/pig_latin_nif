defmodule PigLatinNifTest do
  use ExUnit.Case
  doctest PigLatinNif

  test "greets the world" do
    assert PigLatinNif.hello() == :world
  end
end
