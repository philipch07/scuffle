#!/bin/bash
if [ -n "$3" ]
    then
        sed -i "s/<\/nav><div class=\"sidebar-resizer\"/<div class=\"version\">Deployed from<br><a href=\"https:\/\/github.com\/ScuffleCloud\/scuffle\/pull\/$3\">Pull Request $3<\/a><br><a href=\"https:\/\/github.com\/ScuffleCloud\/scuffle\/commit\/$1\">Commit <code>$2<\/code><\/a><\/div><\/nav><div class=\"sidebar-resizer\"/" target/doc/index.html
    else
        sed -i "s/<\/nav><div class=\"sidebar-resizer\"/<div class=\"version\">Deployed from<br><a href=\"https:\/\/github.com\/ScuffleCloud\/scuffle\/commit\/$1\">Commit <code>$2<\/code><\/a><\/div><\/nav><div class=\"sidebar-resizer\"/" target/doc/index.html
fi
