babash
======

An tiny wrapper around `std::process::Command` sacrificing performance for simplicity.

- works on both linux and windows
- assumes an `utf-8` environment
- logs all commands and responses with proper levels

Example
-------

```rust
fn main() -> std::io::Result<()> {
    stderrlog::new().verbosity(4).init().unwrap();
    let r = babash::call("foo")?; // doesn't check for exit status
    assert_eq!(r.code, Some(127));
    babash::ensure_call("bar")?; // produces an error on an bad exit status
    Ok(())
}
```

produces the following logging output:

```
INFO - Calling: foo
DEBUG - Response {
    command: "foo",
    code: Some(
        127,
    ),
    success: false,
    stdout: "",
    stderr: "sh: 1: foo: not found\n",
}
INFO - Calling: bar
ERROR - Response {
    command: "bar",
    code: Some(
        127,
    ),
    success: false,
    stdout: "",
    stderr: "sh: 1: bar: not found\n",
}
Error: Custom { kind: Other, error: "Unsuccessful call: Response {\n    command: \"bar\",\n    code: Some(\n
  127,\n    ),\n    success: false,\n    stdout: \"\",\n    stderr: \"sh: 1: bar: not found\\n\",\n}" }
```
