# Boomerang... cuz yk, a boomerang always comes back

This project provides [an http](https://www.reddit.com/r/webdev/comments/1i6qzgj/does_anyone_else_say_a_http/) server which will provide back an IP and a [User-Agent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/User-Agent).

### Why use this?
* **No `ipify` required:** Many lightweight IP echo tools secretly proxy requests out to thirdparty APIs, like `ipify`, `icanhazip`. Boomerang on the otherhand reads directly from incoming sockets and proxy headers.
* **Zero foreign calls:** 100% self contained with no outbound network dependencies
* **No tracking:** Zero telemetry, logging, or thirdparty analytics.

I love you
