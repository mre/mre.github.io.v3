+++
title = "Adding AVIF Support to my Blog"
date = 2020-09-02
draft =true
+++

Did I mention that this website is fast?
Oh yeah, [I did](/2019/tinysearch/). [Multiple times](/2017/image-previews/).

Few reasons (from simple to crazy):

- Static site
- Cached on Cloudflare CDN
- HTTP/2 and HTTP/3 support
- No web fonts (sadly)
- No Google Analytics (got [something better...](https://jorgelbg.me/dashflare/))
- Avoid JavaScript whenever possible; CSS covers 90% of my use-cases
- Inlined, optimized SVG graphics and CSS
- [Static WASM search](https://github.com/tinysearch/tinysearch) (lazy loaded)
- Entire homepage is <10K (brotli-compressed) including graphics, thus should fit into the [first HTTP round-trip](https://www.tunetheweb.com/blog/critical-resources-and-the-first-14kb/).
- Heck, even the favicon is optimized for size

Then again it's 2020: **everyone** is optimizing their favicons, right? [...right!?](http://www.p01.org/defender_of_the_favicon/)

Well, turns out most other sites don't think about their user's data plans as much as I do. Actually that's an understatement: they don't care at all.  
But to me, **lean is beautiful**!

## What About Images?

I use SVG whenever possible for diagrams and illustrations.
Only if it's a photo I'll use JPEG or [WebP](https://developers.google.com/speed/webp/).

To be honest with you, my dear reader, I never really liked WebP.
It was [forced upon us by Chrome](https://bugzilla.mozilla.org/show_bug.cgi?id=856375) and it might not even be smaller than JPEGs compressed with [MozJPEG](https://siipo.la/blog/is-webp-really-better-than-jpeg)!
That entire thing was a marketing stunt and as of today Safari [still doesn't support it](https://caniuse.com/#search=webp).

Meet [AVIF](https://aomediacodec.github.io/av1-avif/), the new next-gen image compression format.
If WebP was the empire, AVIF would be the rebellion.
Open, peer-reviewed, no licensing fees &mdash; yup, just like the rebellion.

## Just Skip The Chase Already

Okay okay. Look at this!

https://twitter.com/smashingmag/status/1297907612898471942

– 50% savings compared to JPEG
– 20% savings compared to WebP
– Supported since Chrome 85 and Firefox 80

[It hit me like a hurricane](https://www.youtube.com/watch?v=BixwVsiDdZM):

{% info() %}
Holy smokes, AVIF is supported by major browsers now!? 😲  
I want this for my blog!
{% end %}

[AVIF support for Zola](https://github.com/image-rs/image/issues/1152) is not quite there yet, but I want this now!
So I whipped up an [ugly Rust script](https://github.com/mre/mre.github.io/tree/source/helpers/img) (as I do) that creates AVIF images from my old JPEG and PNG images.

Under the hood it uses [cavif](https://github.com/kornelski/cavif) by [kornelski](https://github.com/kornelski).
(Yeah, it also creates webp as a fallback &mdash; just in case.)

## Performance Results

TODO

- Screenshot of Webpagetest before and after

## Fallback For Older Browsers

HTML is great in that your browser ignores unknown new syntax.
So I can use the `<picture>` element to serve the right format to you. (Look ma, no JavaScript!)

```html
<picture>
  <source srcset="fancy_browser.avif" />
  <source srcset="decent_browser.webp" />
  <img src="meh_browser.jpg" />
</picture>
```

[The real
thing](https://github.com/mre/mre.github.io/blob/source/templates/shortcodes/figure.html)
is a bit more convoluted, but you get the idea. Images even get lazy-loaded
thanks to the new
[`loading=lazy`](https://developer.mozilla.org/en-US/docs/Web/Performance/Lazy_loading)
HTML attribute. 🥳

But hold on for a sec... is your browser actually capable of using AVIF?

BROWSER SUPPORT BUTTON HERE.

If that reads "yup", you're all set.  
If that reads "nope", then you have a few options:

- **On Firefox**: Open `about:config` from the address bar and search for `avif`.
- **On Chrome**: Make sure to update to the latest version.
- **On Safari**: I'm not sure what you're doing with your life. Try a real browser instead. 😏

So there you have it! AVIF from the future is here today.
Add it to your sites as well so that you save me some bandwidth next time I come by.
