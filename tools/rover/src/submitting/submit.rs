use crate::{launch_git, repository::repo::Repository};
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    process,
};

pub fn submit_problem(
    problem_path: &Path,
    message: &str,
    solutions_repo: Option<PathBuf>,
) -> Result<()> {
    let repository = Repository::from_path(problem_path)?;
    let problem = repository.problem_from_path(problem_path)?;
    let solutions_repo = match solutions_repo {
        Some(path) => path,
        None => repository.solutions_repo()?,
    };
    problem.move_solution_files_to(&solutions_repo)?;
    launch_git!(&solutions_repo, "git add failed", "add", ".");
    launch_git!(
        &solutions_repo,
        "git commit failed",
        "commit",
        "-m",
        message
    );
    launch_git!(&solutions_repo, "git push failed", "push");
    Ok(())
}
