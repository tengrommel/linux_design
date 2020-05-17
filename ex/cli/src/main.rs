// some-old-feature
// what do to?(k/d/s/?) > s
use chrono::prelude::*;
use chrono::Duration;
use git2::{Oid,BranchType};
use git2::Repository;
use std::string::FromUtf8Error;
use std::io;
use std::io::{Read, Write};

struct Foo {
    y: &'static str,
}

fn main() -> Result<(), Error> {
    let repo = Repository::open_from_env()?;
    Ok(())
}

type Result<T, E=Error> = std::result::Result<T, E>;

fn get_branches(repo: &Repository) -> Result<Vec<Branch>> {
    let mut branches = vec![];
    for branch in repo.branches(Some(BranchType::Local))? {
        let (branch, branch_type) = branch?;
        let name = String::from_utf8(branch.name_bytes()?.to_vec())?;

        let commit = branch.get().peel_to_commit()?;
        let time = commit.time();
        let offset = Duration::minutes(i64::from(time.offset_minutes()));
        let time = NaiveDateTime::from_timestamp(time.seconds(), 0) + offset;
        branches.push(Branch{
            id: commit.id(),
            time,
            name
        });
    }
    Ok(branches)
}

#[derive(Debug)]
struct Branch {
    id: Oid,
    time: NaiveDateTime,
    name: String
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    GitError(#[from] git2::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}

// impl From<crossterm::ErrorKind> for Error {
//     fn from(error: crossterm::ErrorKind) -> Error {
//         Error::CrosstermError(error)
//     }
// }

// impl From<io::Error> for Error {
//     fn from(error: io::Error) -> Error {
//         Error::IoError(error)
//     }
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Error::CrosstermError(inner) => write!(f, "{}", inner),
//             Error::IoError(inner) => write!(f, "{}", inner),
//         }
//     }
// }

// impl std::error::Error for Error {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             Error::CrosstermError(inner) => Some(inner),
//             Error::IoError(inner) => Some(inner),
//         }
//     }
// }
