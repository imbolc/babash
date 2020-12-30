fn main() -> std::io::Result<()> {
    stderrlog::new().verbosity(4).init().unwrap();
    let r = babash::call("foo")?; // doesn't check for exit status
    assert_eq!(r.code, Some(127));
    babash::ensure_call("bar")?; // produces an error on an bad exit status
    Ok(())
}
