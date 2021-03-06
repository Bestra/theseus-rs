extern crate git2;
extern crate docopt;
extern crate time;

use git2::{Repository, Error, BranchType};
use docopt::Docopt;
use std::collections::HashMap;

fn main() {
    const USAGE: &'static str = "
usage: hello_world <path>
";

    let doc_args: docopt::ArgvMap = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());


    let path = doc_args.get_str("<path>");
    open_repo(path);
}

fn commit_cohort(commit: &git2::Commit) -> String {
    let cohort_format = "%m %Y";
    let s = commit.time().seconds().to_string();
    let commit_time = time::strptime(&s, "%s").unwrap();
    let fmt_time = time::strftime(cohort_format, &commit_time).unwrap();
    return fmt_time

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
        let mut commit = try!(repo.find_commit(id));
        let cohort_str = commit_cohort(&commit);
        let summary = commit.summary().unwrap();
        println!("{} at {}", summary, cohort_str);
    }

    Ok(())
}
