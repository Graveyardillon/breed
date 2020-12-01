# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :polly,
  ecto_repos: [Polly.Repo]

# Configures the endpoint
config :polly, PollyWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "GZWXn9L8U+NWOyhlmS7uMfZsjjkxDA6SMvuJbWGKKjzj4P76x3/ncJxDmhMSXRE5",
  render_errors: [view: PollyWeb.ErrorView, accepts: ~w(html json), layout: false],
  pubsub_server: Polly.PubSub,
  live_view: [signing_salt: "nX4vhDs5"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
