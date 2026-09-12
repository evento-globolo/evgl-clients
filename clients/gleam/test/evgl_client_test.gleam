import evgl_client
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

pub fn health_endpoint_test() {
  evgl_client.health("https://api.example.com")
  |> should.equal("https://api.example.com/healthz")
}
