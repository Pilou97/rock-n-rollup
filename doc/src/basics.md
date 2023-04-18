# Basics

`rock-n-rollup` uses the same semantic as major libraries of the Rust ecosystem. You can register transitions to the application (as handler for actix, or system for bevy, or route for axum, etc...).

This transition can take any parameters that implement the trait `FromInput`.

In this section, you will learn and discover how to write custom transitions to fullfill your needs.
