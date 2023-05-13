# Plugins

rock-n-rollup provides many plugins to ease your developement.

First question, what is a plugin? A plugin is an **augmented** runtime. Basically a plugin is a superset of the runtime.

## How to use plugins?

Basically a plugin is a trait that is implemented for any runtime. So when you want to use the plugin you can add a constraint on the `Runtime`:

```rust, noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::core::Runtime;
use rock_n_rollup::plugins::logger::Logger;

fn transition<R: Runtime + Logger>(rt: &mut R) {
    rt.log("Hello kernel");
}
# fn main() {}
```

If you don't care of the `Runtime` you can restrict your function to your plugin and only your plugin:

```rust, noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::logger::Logger;

fn transition<R: Logger>(rt: &mut R) {
    rt.log("Hello kernel");
}
# fn main() {}
```

And of course, you can compose plugins with each other

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::logger::Logger;
use rock_n_rollup::plugins::hasher::Hasher;


fn transition<R: Logger + Hasher >(rt: &mut R) {
    let data = "Hello world";
    rt.info("Hello world");
    let hash = rt.hash(data.as_bytes());
}
# fn main() {}
```

Unfortunately there is one limitation, it's complicated to use two plugins that exposes the same feature.

## How to develop a plugin?

Let's say you want to implement your custom plugin, you can implement your plugin in 3 steps:

1. Define a trait

This trait can define any function.
Let's implement the identity plugin: it returns the given parameter.

```rust
trait Identity {
    fn identity<P>(&mut self, param: P) -> P;
}
# fn main(){}
```

2. Implement this trait for any Runtime

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::core::Runtime;
# trait Identity {
#    fn identity<P>(&mut self, param: P) -> P;
# }

impl <R> Identity for R
where
    R: Runtime
{
    fn identity<P>(&mut self, param: P) -> P {
        param
    }
}
# fn main(){}
```

Tips, you can also compose plugins

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::core::Runtime;
use rock_n_rollup::plugins::logger::Logger;
# trait Identity {
#    fn identity<P>(&mut self, param: P) -> P;
# }

impl <R> Identity for R
where
    R: Runtime + Logger
{
    fn identity<P>(&mut self, param: P) -> P {
        self.log("Hello identity plugin");
        param
    }
}
# fn main(){}
```

3. Implement some test

Because you have implemented your trait for all `Runtime`, you can directly use the `MockRuntime` to test your plugin.

```rust
# extern crate rock_n_rollup;
# use rock_n_rollup::core::Runtime;
# use rock_n_rollup::plugins::logger::Logger;
# trait Identity {
#    fn identity<P>(&mut self, param: P) -> P;
# }
# impl <R> Identity for R
# where
#    R: Runtime + Logger
# {
#    fn identity<P>(&mut self, param: P) -> P {
#        self.log("Hello identity plugin");
#        param
#    }
# }
use rock_n_rollup::core::MockRuntime;

fn test() {
    let mut runtime = MockRuntime::default();
    let param = runtime.identity(32);
    assert_eq!(param, 32)
}
# fn main(){}
```
