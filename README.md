# Docker sshfs file

This repo aim to test the sshfs performance on large file that require a lots
of reading .  
normally sshfs cache the file, so there shouldn't be too much slow down in reading.


Run `generate_number.py` to create a large file so we can test read speed for further analysis.

1. mount a volume to `/home/data`.
2. run the app like `./app -c generate -f file.txt -q 100`. this will generate 100 line of txt to search on
3. run `./app -c search -f file.txt -q 42` will search for that number in the file
