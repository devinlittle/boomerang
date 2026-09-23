# Boomerang... cuz yk, a boomerang always comes back

This project provides [an http](https://www.reddit.com/r/webdev/comments/1i6qzgj/does_anyone_else_say_a_http/) server which will provide back an your Public IP and a [User-Agent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/User-Agent).

## Example Output: 
```bash
$ curl https://ip.devinlittle.net
24.218.230.104

User-Agent: curl/8.22.0
```

This can be useful when you would like to know your IP address not just from a browser, but from the CLI.
This server doesn't track/store IP information OR use thirdparty services (like ipify, canihazip, ect) to get IP information.
