[![Crates.io](https://img.shields.io/crates/v/simplyplural.svg)](https://crates.io/crates/simplyplural)
[![License](https://img.shields.io/crates/l/simplyplural)](LICENSE)


# SimplyPlural Api Client

> This rust library provides a client for interacting with the Web Api of the SimplyPlural app.  
> The client is fully async and backed by [reqwest](https://crates.io/crates/reqwest).

You can create the SPClient with just your API token and the environment in which you're running.
We recommend testing everything in [the development environment](https://devapp.apparyllis.com) first!
You can also provide your custom configured http client, should you not want to use the default reqwest client.

## Features

Note that each group of endpoints, as specified by [the documentation](https://docs.apparyllis.com/docs/docs/api), has it's own feature.  
Only the `users` feature is enabled by default, so please make sure to enable any additional features you require.

## Example
This is a minimalistic example for printing all the names of the currently fronting members:
```rs
use std::env::var;
use reqwest::Error;
use crate::{SPClient, SPEnvironment};

async fn main() {
    let token = var("SP_TOKEN").expect("Expected an api token");

    // Here we create a client for the development environment
    let client = SPClient::new(token, SPEnvironment::Development);

    if let Err(err) = print_current_fronters(&client).await {
        eprintln!("Error: {}", err);
    }
}

async fn print_current_fronters(client: &SPClient) -> Result<(), Error> {
    let my_id = client.get_self_user().await?.id; // get our own user id

    let fronters = client.get_current_fronters().await?; // get the current fronters for our own system
    for fronter in fronters {
        if let Some(content) = fronter.content {

            // get the member info for each fronter (you might want to cache this in a real application if it runs longer)
            if let Ok(member) = client.get_member(&my_id, content.member).await {
                if let Some(member_content) = member.content {
                    // in this example we just print the member name
                    // but you can access other member fields as needed
                    println!("Fronter: {}", member_content.name);
                }
            }
        }
    }

    Ok(())
}
```

## Installation
Add `simplyplural` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
simplyplural = { version = "*", features = ["users"] }
tokio = { version = "*", features = ["rt-multi-thread"] }
```

Any async runtime will work, here we just use `tokio` as an example.

## License

Licensed under the [MIT license](https://github.com/Web-44/simplyplural-rs/blob/master/LICENSE)