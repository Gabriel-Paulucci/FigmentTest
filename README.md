# Reproduction

Windows
```ps
$env:APP_SECRET = '123'; cargo run
```

Linux
```sh
APP_SECRET=123 cargo run
```

Error
```rs
Error {
    tag: Tag(Global, 1),
    profile: Some(Profile(Uncased { string: "global" })), metadata: Some(Metadata {
        name: "`APP_` environment variable(s)",
        source: None,
        provide_location: Some(Location {
            file: "src\\config.rs",
            line: 10,
            col: 24
        }),
        interpolater:
    }),
    path: ["secret"],
    kind: InvalidType(Unsigned(123), "a string"),
    prev: None
}
```
