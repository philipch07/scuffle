#!/bin/bash
if [ -n "$4" ]
    then
        sed -i "s#</nav><div class=\"sidebar-resizer\"#<div class=\"version\">Deployed from<br><a href=\"$1/pull/$4\">Pull Request $4</a><br><a href=\"$1/commit/$2\">Commit <code>$3</code></a></div></nav><div class=\"sidebar-resizer\"#" target/doc/index.html
    else
        sed -i "s#</nav><div class=\"sidebar-resizer\"#<div class=\"version\">Deployed from<br><a href=\"$1/commit/$2\">Commit <code>$3</code></a></div></nav><div class=\"sidebar-resizer\"#" target/doc/index.html
fi
