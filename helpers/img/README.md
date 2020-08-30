# Process images

- creates webp and avif images from PNG/JPG
- puts all images into a folder in `static` to help with caching the path
  (e.g. `*/static/img/*` as a Cloudflare rule).

## Pseudocode

```python
for dir in content/**
    in_dir = basename(dir)
    out_dir = static/in_dir
    for image in dir/raw
        if svg:
            cp image to out_dir
            continue
        adjust width
        create webp
        create avif
        cp image to out_dir
        cp image.webp to out_dir
        cp image.avif to out_dir
```
