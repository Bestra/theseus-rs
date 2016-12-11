extern crate git2;
use git2::{Repository, Error, BranchType};

fn main() {
    println!("Hello, world!");
}

fn open_repo(path: &str) -> Result<(), Error> {
    let repo = try!(Repository::open(path));
    let branch = try!(repo.find_branch("master", BranchType::Local));

    let mut revwalk = try!(repo.revwalk());
    revwalk.set_sorting(git2::SORT_TOPOLOGICAL);

    Ok(())
}
