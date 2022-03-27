# luana

lua packager for lua5.2+ (with support for some luau features)

## features
currently has support for 
- minification
- importing from other files as text and data structures
- -  json to table
- importing other code into one file.

Look at lua_test folder to see what it looks like.

<hr>

this repo is still under development, so there is a few bugs which will eventually be fixed (?maybe)

## getting started

To get started go to the [releases](https://github.com/pozm/luana/releases) tab and download the latest release.
once you have done that you will probably want to add the exe to your path, although it is not required it is alot easier to use the tool and is recommended.

from there you can run `luana -h` to get help.

for where you need to provide a path you can provide relative paths such as `.`, `./proj`, etc.

### making a new project

run `luana init <project name> <?path>` to create a new luana project!

### building project

run `luana build <path>` to build the project.

## other cool package managers/bundlers 😎

- [Tape](https://github.com/Belkworks/tape) by [safazi](https://github.com/safazi)
- [lua-pack](https://github.com/Bork0038/lua-pack) by [bork](https://github.com/Bork0038)
- [lua-bundler](https://github.com/yatyricky/lua-bundler) by [yatyricky](https://github.com/yatyricky)
- [luabundler](https://github.com/Benjamin-Dobell/luabundler) by [Benjamin-Dobell](https://github.com/Benjamin-Dobell)
