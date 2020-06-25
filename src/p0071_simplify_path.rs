// 71. Simplify Path
// Medium

// Given an absolute path for a file (Unix-style), simplify it. Or in other words, convert it to the canonical path.

// In a UNIX-style file system, a period . refers to the current directory. Furthermore, a double period .. moves the directory up a level.

// Note that the returned canonical path must always begin with a slash /, and there must be only a single slash / between two directory names. The last directory name (if it exists) must not end with a trailing /. Also, the canonical path must be the shortest string representing the absolute path.

 

// Example 1:

// Input: "/home/"
// Output: "/home"
// Explanation: Note that there is no trailing slash after the last directory name.
// Example 2:

// Input: "/../"
// Output: "/"
// Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
// Example 3:

// Input: "/home//foo/"
// Output: "/home/foo"
// Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
// Example 4:

// Input: "/a/./b/../../c/"
// Output: "/c"
// Example 5:

// Input: "/a/../../b/../c//.//"
// Output: "/c"
// Example 6:

// Input: "/a//b////c/d//././/.."
// Output: "/a/b/c"

pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];
        for part in path.split('/') {
            if part == "..".to_owned() {
                stack.pop();
            } else if !(part == ".".to_owned() || part == "".to_owned()) {
                stack.push(part);
            }
        }
        "/".to_owned() + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_path_test() {
        assert_eq!(Solution::simplify_path("/home/".to_owned()), "/home".to_owned());
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/".to_owned());
        assert_eq!(Solution::simplify_path("/home//foo/".to_owned()), "/home/foo".to_owned());
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_owned()), "/c".to_owned());

        assert_eq!(Solution::simplify_path("/.hidden".to_owned()), "/.hidden".to_owned());
        assert_eq!(Solution::simplify_path("/..hidden".to_owned()), "/..hidden".to_owned());
        assert_eq!(Solution::simplify_path("/abc/...".to_owned()), "/abc/...".to_owned());
        assert_eq!(Solution::simplify_path("/...".to_owned()), "/...".to_owned());
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/".to_owned());
    }
}
