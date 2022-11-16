## URL Image Perceptual Hasher

This is a quick script to retrieve images from a provided file where each
image URL is on its own line, then generate the perceptual hash for them.
It writes a comma-separated listing of the base64-encoded perceptual hash
(phash) and the original URL out to stdout. Not much error handling has
been added here, so if the image can't be retrieved a blank value will be
output for the phash.

Since IO wait is likely to be the bottleneck, this program runs multiple threads,
defaulting to the number of cores of the system running it.

An example input file is provided in `example.txt`.

An example of the usage and output is:

```
$ ./url_image_hasher --input=example.txt
FiMn15QyYNE=,https://plus.unsplash.com/premium_photo-1664372599591-9fe3894fa013?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=802&q=80
MzU2FloJDU0=,https://images.unsplash.com/photo-1668342448695-1e5256d149d4?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1769&q=80
```

### Running

To run, you can [install rust](https://rustup.rs/) (if it's not installed
already) and use the following commands:

```
cargo build --release
./target/release/url_image_hasher --input=<urls-input-filename.txt>
```
