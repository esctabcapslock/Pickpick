<div align="center">
<img src="./doc/icon.png">

# Pickpick
Calculate the exact time of the web server.
<!-- ![](./doc/test-domain.gif) -->

<img src="./doc/test-domain.gif">
</div>

## Features
- Use the Date header in the http response to calculate the time of the server.
- A total of 16 requests are sent to web server.
- It assumed that this value was lowered in seconds.
- The default DNS server is `1.1.1.1`
- GUI by [Iced](https://github.com/iced-rs/iced), Date parsing by [Chrono](https://github.com/chronotope/chrono), Url parsing by [Regex](https://github.com/rust-lang/regex).
- All requests are sent unencrypted via [TCP socket](https://doc.rust-lang.org/std/net/index.html)

## Downloads
- Windows 10 only
- Use [releases](https://github.com/esctabcapslock/pickpick/releases) page in this repo.
