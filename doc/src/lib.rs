#[cfg(test)]
extern crate skeptic;

#[cfg(test)]
mod tests {
    include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
}
