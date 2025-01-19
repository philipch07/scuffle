#!/bin/bash
set -eo pipefail

repo_url=$1
commit_hash=$2
short_commit_hash=$3
pull_request_number=${4:-}

pull_request_code=""
if [ -n "$pull_request_number" ]; then
    pull_request_code="<br><a href=\"$repo_url/pull/$pull_request_number\">Pull Request $pull_request_number</a>"
fi

commit_code="<br><a href=\"$repo_url/commit/$commit_hash\">Commit <code>$short_commit_hash</code></a>"

sed -i "s#</nav><div class=\"sidebar-resizer\"#<div class=\"version\">Deployed from$pull_request_code$commit_code</div></nav><div class=\"sidebar-resizer\"#" target/doc/index.html
