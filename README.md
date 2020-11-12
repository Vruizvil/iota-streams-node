# Introduction

npm module to run rust based iota streams in a javascript based environment

# Directory Structure

Code under `native` is rust code. Code under `lib` is javascript.

# Extend

After modifying rust code under `native/src/`, build using `neon build` and voila; your code is ready to be used in javascript. Javascript code is available under lib can now directly access the compiled module as in example `lib/index.js`

    neon build
    node lib/index.js

And that's it!
