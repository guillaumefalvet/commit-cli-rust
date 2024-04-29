use std::process;
// - Function to parse the current Git branch
pub fn parse_git_branch() -> Option<String> {
  let output = process::Command::new("git")
    .args(&["branch", "--show-current"])
    .output()
    .ok()?;
  String::from_utf8(output.stdout).ok()
}


// - Function to commit the changes
pub fn commit(commit_message: String) -> Option<String>{
  let output = process::Command::new("git")
    .args(&["commit", "-m", &commit_message])
    .output()
    .ok()?;
  String::from_utf8(output.stdout).ok()
}