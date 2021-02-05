# Rocket.rs Static File Fallthroug

This small projects tests the file fallthrough for [rocket_contrib::serve::StaticFiles](https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html).

Run `cargo test` to execute the tests.

The small server will expose one enpoint `/static`. Behind that endpoint, `StaticFiles` should serve the content from two folders (`folder1` and `folder2`).

- `folder1` -> `file1.txt` -> `hi from file 1`
- `folder2` -> `file2.txt` -> `hi from file 2`

See https://github.com/SergioBenitez/Rocket/issues/1526.
