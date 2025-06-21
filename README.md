# Rustblox

A Rust library for interacting with the Roblox API

# Project Status

So the last update I made here, I said I had "found time" and
then proceeded to not work on this too much. I won't make guarantees,
but I may continue to work on this.

---

## Features

Rustblox intents to support as many of the Roblox API endpoints as possible.
Most of the Roblox API has *good enough* documentation, but it is possible that
some things will be missed or fall through the cracks, especially in regard to
`realtime.roblox.com` (as that has little documentation that I've found).

Rustblox will primarily try to support endpoints that are useful to automated accounts.
Authentication will be done via the .ROBLOSECURITY cookie. See [here](#roblosecurity-cookies)
for more information.

## Getting Started

To get started, you need a `RustbloxClient`, which you can get from a `RustbloxClientBuilder`.

```rust
// -- snip --
use rustblox::builder::RustbloxClientBuilder;

let client = RustbloxClientBuilder::new()
    .build();
// -- snip --
```

This will build a client without any authentication capabilities. If you desire
the ability to use authentication, you will need a [.ROBLOSECURITY cookie](#roblosecurity-cookies).
Once you have one, you can build a `RustbloxClient` as follows:

```rust
// -- snip --
use rustblox::builder::RustbloxClientBuilder;

let mut client = RustbloxClientBuilder::new()
    .insert_cookie("<YOUR COOKIE>")
    .build();
let login_result = client
    .login()
    .await
    .unwrap();
// -- snip --
```

Once you have a `RustbloxClient`, you can make requests to the Roblox API using the client directly.
Eventually, feature flags will gate which API functions are added in.

``` rust
// -- snip --
let client = RustbloxClientBuilder::new().build();
let result = client.get_user_info(1).await;
println!("{:#?}", result);
// -- snip --
```

If using an authenticated client, the default behavior is to return an Err on any failure to authenticate.
If you'd like to automatically re-attempt (once) to refresh your X-CSRF-TOKEN, you can use the
`automatic_reauthentication()` method on the `RustbloxClientBuilder` to set your preference.

## MSRV (Minimum Supported Rust Version)
The current MSRV is 1.63.0.

## Notes

### .ROBLOSECURITY Cookies

Roblox has (justifiably, arguably) made it a bit difficult to authenticate with their API.
The easiest and cleanest way to authenticate is by logging in via your browser and retrieving
your .ROBLOSECURITY cookie. How do you do this?
1) Log in like normal
2) Open up the developer tools (right-click then "Inspect"/"Inspect Element" is easiest)
3) Go to the "Storage" tab and then find wherever the cookies are stored
4) The .ROBLOSECURITY cookie will be under there somewhere. It's a very big cookie.
And please listen to Roblox -- don't share it with anyone unless you know what you're doing.

**SPECIAL NOTE FOR THOSE USING A VIRTUAL PRIVATE SERVER (VPS)**
Roblox added IP region tracking to .ROBLOSECURITY cookies some time ago, which means that if
a particular .ROBLOSECURITY cookie is used to log in from a location different from the one
it was generated in, the cookie will become invalid. 
[This issue in noblox.js](https://github.com/noblox/noblox.js/issues/545) contains a great 
tutorial on how to bypass this.

### When will this be finished?

I honestly have no clue. I'm just a college student who wanted to make this for no real 
particular reason. I've also got some other small projects I'm working on. If you want to 
see this finished sooner, feel free to help out and make a pull request!
