Proxy listens on "127.0.0.1:8080" when you do `cargo run`.

The proxy always returns a 403 because there are no upstreams to proxy to. However it also logs the first element in the path.
