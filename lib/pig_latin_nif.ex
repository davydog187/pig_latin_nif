defmodule PigLatinNIF do
  use Rustler, otp_app: :pig_latin_nif, crate: "pig_latin_nif"

  def translate(_word) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
