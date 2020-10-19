use std::process::Command;

pub struct GitDataInjector{}
impl GitDataInjector{

    pub fn new() -> Self{
        GitDataInjector{}
    }

    pub fn with_last_commit_revision_hash(self) -> Self{
        let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("DataInjector error: impossible to retrieve GIT last commit revision hash.");

        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);

        self
    }

    pub fn with_last_commit_message(self) -> Self{

        let output = Command::new("git")
        .args(&["log", "-1", "--pretty=format:%B"])
        .output()
        .expect("DataInjector error: impossible to retrieve GIT last commit message.");

        let git_message = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_MESSAGE={}", git_message);

        self
    }

    pub fn with_last_commit_date(self) -> Self{

        let output = Command::new("git")
        .args(&["log", "-1", "--pretty=format:%cd"])
        .output()
        .expect("DataInjector error: impossible to retrieve GIT last commit date.");

        let git_date = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_MESSAGE={}", git_date);

        self
    }
}

