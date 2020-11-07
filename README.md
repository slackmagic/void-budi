# 🦀🔠 void-BUild-Data-Injector (BUDI)

Inject information to a RUST build (LIB/EXE) at compile time.

_limited to git for the moment_

Add this dependency to your _cargo.toml_:

```
[build-dependencies]
void-budi = {git = "https://github.com/slackmagic/void-budi", branch = "master"}
```

Add this code to your _build.rs_ file (if not present create it on your project root folder):


```
use void_budi::*;
fn main() {
    GitDataInjector::new()
    .with_last_commit_revision_hash()
    .with_last_commit_short_revision_hash()
    .with_last_commit_message()
    .with_last_commit_date();
}
```

Into your code :

```
use std::env;

fn main() {
    println!("{:?}", env!("GIT_HASH"));
    println!("{:?}", env!("GIT_SHORT_HASH"));
    println!("{:?}", env!("GIT_MESSAGE"));
    println!("{:?}", env!("GIT_DATE"));
}
```
