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

```bash
curl http://localhost:1054/api
```

Should reply "Hey there!"

```bash
curl http://localhost:1054
```

should hand for 10 seconds

and if you POST on route `/echo`...

```
curl -X POST -d "Do you here me?"  http://localhost:1054/echo
```

try it!
