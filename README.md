# lagging_server

A simple server that waits for 10 seconds before responding.

Using Actix for this is insanely overkill, but here we are: two lines of code.

Also works in NodeJS but I hate javascript:

```js
const http = require("http");
http
  .createServer(function (req, res) {
    setTimeout(function () {
      res.writeHead(204, {});
      res.end();
    }, 10_000);
  })
  .listen(process.env.PORT || 8080);
```

## Build

```bash
cargo install --path .
```

## Run

```
lagging_server --port 1054
```

You can run several ones, in several terminals, for instance:

```bash
lagging_server --port 1052
```

_Et cætera_

## Request it with curl

| curl command                                                    | outcome                                         |
| :-------------------------------------------------------------- | :---------------------------------------------- |
| `curl http://localhost:1054/api`                                | replies "Hey there!"                            |
| `curl http://localhost:1054/latency`                            | replies after a latency period, tells about it  |
| `curl http://localhost:1054`                                    | should hang for 10 seconds                      |
| `curl -X POST -d "Do you hear me?"  http://localhost:1054/echo` | echoes back to you                              |

Try it!
