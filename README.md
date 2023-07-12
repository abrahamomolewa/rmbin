# rmbin


The rmbin script allows you to easily move files to the Trash (recycle bin) on your Mac.

Installation
To install and use rmbin on your Mac, follow these steps:

Open Terminal on your Mac.

Clone the rmbin repository to your local machine:

  `` git clone https://github.com/abrahamomolewa/rmbin.git  ``

Build the rmbin script using cargo:

`` Cargo build --release  ``

Copy the built executable to a directory in your system's PATH, such as /usr/local/bin:

`` cp target/release/rmbin /usr/local/bin/rmbin ``

Verify the installation by running rmbin:

`` rmbin --help ``


# usage

To use rmbin, simply provide the file path(s) you want to move to the Trash as command-line arguments. For example:

`` rmbin file1.txt file2.txt   ``

This will move file1.txt and file2.txt to the Trash.
