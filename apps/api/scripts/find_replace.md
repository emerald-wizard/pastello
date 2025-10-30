Great idea—switching to **only tracked files** is clean with `git ls-files`. Here’s a tight version that:

* works inside a Git repo
* takes **from**, **to**, and one or more **globs/pathspecs** (e.g., `"*.go"` or `"**/*.txt"`)
* exports env vars so the whole pipeline sees them
* no changes if nothing matches

```bash
# ~/.zshrc or ~/.bashrc
# Usage:
#   freplace "from" "to" "*.txt"
#   freplace "from" "to" "*.go" "*.mod"
#   freplace "from" "to" "src/**/*.ts"
freplace() {
  local from="$1" to="$2"
  shift 2

  if [ -z "$from" ] || [ -z "$to" ] || [ $# -lt 1 ]; then
    echo 'Error: need "from", "to", and at least one glob/pathspec.' >&2
    echo 'Usage: freplace "from" "to" "*.txt" [more globs...]' >&2
    return 1
  fi

  # Must be inside a git repo
  if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "Error: not inside a Git repository." >&2
    return 1
  fi

  # If no tracked files match, bail quietly
  if ! git ls-files -- "$@" | grep -q . ; then
    echo "No tracked files match: $*" >&2
    return 0
  fi

  export FROM="$from" TO="$to"

  # Edit only tracked files that match the given pathspecs
  git ls-files -z -- "$@" \
    | xargs -0 perl -0777 -i -pe '
        BEGIN { our $from = $ENV{FROM}; our $to = $ENV{TO}; }
        s/\Q$from\E/$to/g;
      '

  unset FROM TO
}
```

### Examples

```bash
# Replace across tracked .txt files
freplace "whatever/Iwant" "replace/thisthing" "*.txt"

# Multiple types
freplace "old/pkg" "new/pkg" "*.go" "*.mod"

# Subtrees
freplace "foo" "bar" "cmd/**/*.go" "internal/**/*.go"
```

**Notes**

* Keep the globs **quoted** so the shell doesn’t expand them; Git will match them as pathspecs.
* If you want a quick preview, run:
  `git ls-files -- '*.txt' | xargs grep -nF 'whatever/Iwant'`
