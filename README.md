# I Need A Reference

Are you writing a new page for one of the drivers documentation sets? Do you need an existing,
relatively similar page as a starting point to copy, paste, then tweak accordingly? Look no further.

## Installation

On your command line, navigate to the directory that contains a sub-directory housing all
of your drivers repositories. Then, clone the i-need-a-reference repository.

For example, if your repositories live in a directory called `work-stuff/repositories/`, go
to the `work-stuff` directory and clone this repository.

## Usage

After cloning this repository, navigate into the i-need-a-reference directory and run the following:

`cargo run -- <new file name> <category> <parent directory>`

Edit the placeholders like this:

- Change `<new file name>` to the intended name of your new page, like `aggregation.txt`. Make sure
that your file name ends in `.txt`.
- Change `<category>` to reflect the type of page you're writing, like `fundamentals`. There are only
three available options for this argument: `usage-examples`, `fundamentals`, or `other`.
- Change `<parent directory>` to the name of the directory that holds all your drivers repos, 
like `repositories`.

## Author

Created by Nora Reidy