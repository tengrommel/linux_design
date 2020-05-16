// some-old-feature
// what do to?(k/d/s/?) > s
use std::fmt;
use std::io;
use std::io::{Read, Write};
use git2::Repository;
fn main() -> Result<(), Error> {
    let repo = Repository::open_from_env()?;
    let mut stdout = io::stdout();
    for branch in repo.branches(None)? {
        let (branch, branch_type) = branch?;
        let name = branch.name_bytes()?;
        stdout.write_all(name)?;
        write!(stdout, "\n");
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    GitError(#[from] git2::Error)
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
