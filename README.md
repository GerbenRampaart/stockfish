

# Links
https://stockfishchess.org/download/linux/
https://stackoverflow.com/questions/68630526/lib64-ld-linux-x86-64-so-2-no-such-file-or-directory-error

# Build
```bash
docker build --platform linux/x86_64 . -t stockfish:latest
```

# Run
```bash
docker run --platform linux/x86_64 -it --rm -v $(pwd):/home/sf/docker stockfish:latest
```

NOTE: the --platform is needed because otherwise if you build the image on a m1 if will grab the wrong ubuntu image.