# Rust Webserver
Web server written in Rust

The goal is to learn how to do it with actix and dev a first version of an app with a login feature


# Hot Reload

Hot reload using systemfd and cargo watch is available for faster development.
To enable hot reload, start the server with the following command line

`systemfd --no-pid -s http::8000 -- cargo watch -x run`
