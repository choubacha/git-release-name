# git-release-name-web

This is a small web server that handles requests and will return the specified SHA in the body.

### Usage

From the root directory you can run:
```
$ cargo run -p release-name-web
Listening on http://0.0.0.0:6767
Ctrl-C to shutdown serve
```

### API

```
GET /api/release-name/:sha => text
```

##### Example
```
$ curl "0.0.0.0:6767/api/release-name/a9677113edf998d260e69554dcd4fce200312605"
intentionally mirky swineherds
```
