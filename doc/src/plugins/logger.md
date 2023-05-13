# Logger

The `Logger` plugin is a simple plugin. It adds to the runtime 5 functions:

- `log` to print normal log
- `info` to print log with info level
- `warn` to print log with warn level
- `err` to print log with error level

## How to use the Logger plugin

Let's say you have a `transition`. If you want to use the logger you just add to add the Logger trait to the Runtime constraint:

```rust
use rock_n_rollup::plugins::logger::Logger;

fn transition<R: Logger>(rt: &mut R) {
    rt.log("Normal log with \n at the end");
    rt.info("The log will start by [INFO]");
    rt.warn("The log will start by [WARN]");
    rt.err("The log will start by [ERR]");
}
# fn main(){}
```
