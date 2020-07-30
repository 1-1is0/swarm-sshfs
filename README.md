# Docker sshfs file

This repo aim to test the sshfs performance on large file that require a lots
of reading .  
normally sshfs cache the file, so there shouldn't be too much slow down in reading.

Run `generate_number.py` to create a large file so we can test read speed for further analysis.
