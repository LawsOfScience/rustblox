# Rustblox

A Rust library for interacting with the Roblox API

## Features

Rustblox intents to support as many of the Roblox API endpoints as possible.
Most of the Roblox API has *good enough* documentation, but it is possible that
some things will be missed or fall through the cracks, especially in regard to
`realtime.roblox.com` (as that has little documentation that I've found).

Rustblox will start by working with endpoints that do not require authentication,
and will then implement authentication and support the endpoints that require it.
Authentication will be done via the .ROBLOSECURITY cookie. See [here](#.ROBLOSECURITY-Cookies)
for more information.

## Getting Started

TBD

Will most likely work by creating a `RustbloxClient` and interacting with that.

## MSRV (Minimum Supported Rust Version)
The current MSRV is 1.62.1.

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
