# zap-prettier
Make json logs produced by "zap" and similar json loggers prettier

This is a good starter repo to getting your own very fast JSON log formatter out of your existing apps.

## Usage

Once you've built the binary you can simply pipe to that binary to see improved logging.

```sh
go run main.go | zap-prettier
```
