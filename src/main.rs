extern crate git2;
use git2::{Repository, Error, BranchType};

fn main() {
    println!("Hello, world!");
    open_repo(".");
}

fn open_repo(path: &str) -> Result<(), Error> {
    let repo = try!(Repository::open(path));
    let branch = try!(repo.find_branch("master", BranchType::Local));
    let r = branch.get().target().unwrap();

    let mut revwalk = try!(repo.revwalk());
    revwalk.set_sorting(git2::SORT_TOPOLOGICAL);
    try!(revwalk.push(r));

    for id in revwalk {
        let id = try!(id);
        println!("{}", id);
    }

    Ok(())
}
