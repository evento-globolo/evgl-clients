defmodule EvglClient.MixProject do
  use Mix.Project

  def project do
    [app: :evgl_client, version: "0.1.0", elixir: "~> 1.15", start_permanent: Mix.env() == :prod]
  end

  def application, do: [extra_applications: [:logger, :inets, :ssl]]
end
