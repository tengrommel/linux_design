// some-old-feature
// what do to?(k/d/s/?) > s
use chrono::prelude::*;
use chrono::Duration;
use git2::Repository;
use git2::{BranchType, Oid};
use std::convert::TryFrom;
use std::io;
use std::io::{Stdin, Stdout, Bytes};
use std::io::{Read, Write};
use std::string::FromUtf8Error;

struct Foo {
    y: &'static str,
}

fn main() {
    let result = (|| -> Result<_>{
        let repo = Repository::open_from_env()?;
        crossterm::terminal::enable_raw_mode()?;
    
        let mut stdout = io::stdout();
        let mut stdin = io::stdin().bytes();

        let mut branches = get_branches(&repo)?;

        if branches.is_empty() {
            write!(stdout, "Found no branches (we ignore 'master')\r\n")?;
        }  else {
            for branch in &mut branches {
                match get_branch_action_from_user(&mut stdout, &mut stdin, &branch)? {
                    BranchAction::Quit => {
                        return Ok(())
                    },
                    BranchAction::Keep => {
                        write!(stdout, "\r\n")?;
                    },
                    BranchAction::Delete => {
                        branch.branch.delete()?;
                        write!(stdout, "Deleted branch '{}', to undo run `git branch {} {}`",
                            branch.name,
                            branch.name,
                            branch.id)?;
                    }
                }
            }
        }
        Ok(())  
    })();
    crossterm::terminal::disable_raw_mode().ok();
    match result {
        Ok(()) => {},
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        },
    }
}

fn get_branch_action_from_user(
    stdout: &mut Stdout,
    stdin: &mut Bytes<Stdin>,
    branch: &Branch
) -> Result<BranchAction> {
    write!(
        stdout,
        "'{}' ({}) last commit at {} (s/d/f/g/?)",
        branch.name, &branch.id.to_string()[0..10], branch.time
    )?;
    stdout.flush()?;
    let byte = match stdin.next() {
        Some(byte) => byte?,
        None => return get_branch_action_from_user(stdout, stdin, branch),
    };
    let c = char::from(byte);
    write!(stdout, "{}", )


    if  c == '?' {
            write!(stdout, "\n\r")?;
            write!(stdout, "Here are what the commands mean \r\n")?;
            write!(stdout, "k - Keep the branch\r\n")?;
            write!(stdout, "d - Delete the branch\r\n")?;
            write!(stdout, "q - Quit\r\n")?;
            write!(stdout, "? - Show this help text \r\n")?;
            stdout.flush()?;
            get_branch_action_from_user(stdout, stdin, branch)
    } else {
        let action = BranchAction::try_from(c)?;
        match action {
        BranchAction::Quit => Ok(BranchAction::Quit),
        BranchAction::Keep => Ok(BranchAction::Keep),
        BranchAction::Delete => Ok(BranchAction::Delete)   
    }
    }
    
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn get_branches(repo: &Repository) -> Result<Vec<Branch>> {
    let mut branches = repo
        .branches(Some(BranchType::Local))?
        .map(|branch| {
            let (branch, _) = branch?;
            let name = String::from_utf8(branch.name_bytes()?.to_vec())?;
            let commit = branch.get().peel_to_commit()?;
            let time = commit.time();
            let offset = Duration::minutes(i64::from(time.offset_minutes()));
            let time = NaiveDateTime::from_timestamp(time.seconds(), 0) + offset;
            Ok(Branch {
                id: commit.id(),
                time,
                name,
                branch
            })
        })
        .filter(|branch| {
            branch.as_ref().map(|b| b.name == "master").unwrap_or(true)
        })
        .collect::<Result<Vec<_>>>()?;
    branches.sort_unstable_by_key(|branch| branch.time);
    Ok(branches)
}

struct Branch<'repo> {
    id: Oid,
    time: NaiveDateTime,
    name: String,
    branch: git2::Branch<'repo>
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

    #[error("Invalid input. Don't know what '{0}' means")]
    InvalidInput(char),
}

enum BranchAction {
    Keep,
    Delete,
    Quit,
}

impl TryFrom<char> for BranchAction {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'k' => Ok(BranchAction::Keep),
            'd' => Ok(BranchAction::Delete),
            'q' => Ok(BranchAction::Quit),
            _ => Err(Error::InvalidInput(value)),
        }
    }
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
