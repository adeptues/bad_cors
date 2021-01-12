# bad_cors

This is a terrible implementation of cors support for use in a rust rocket application.

# Usage

To use you must first provide a value to the `Origin` config option as an AdHoc fairing which is used by the
cors fairing to supply the value in the Access-Control-Allow-Origin Header.

```rust
let ignition = rocket::ignite();
let cors = bad_cors::CORS;
ignition = ignition.attach(AdHoc::on_attach("Cross Origin Config", |rocket| {
    let origin =  rocket.config().get_string("origin").expect("No configuration setting for origin");
    let constructed = bad_cors::Origin{origin:origin};
    Ok(rocket.manage(constructed))
    })).attach(cors);
```

you must then provide a config key in your `Rocket.toml` file called origin

```toml
[global]
origin="http://localhost:4200"
```