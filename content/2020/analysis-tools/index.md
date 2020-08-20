+++
title = "Launching a Side Project with Github Sponsors"
date = 2020-08-20
+++

Today we launched [analysis-tools.dev] and boy have I underestimated the
response.

It's a side project about comparing static code analysis tools and it's
[completely open-source](https://github.com/analysis-tools-dev/). We wanted to
build a product that wouldn't depend on showing ads or tracking users. Instead,
we were asking for sponsors on Github, that's it. I learned a lot in the process
and if you like to do the same, keep reading!

## Stats

Everyone likes facts. Here are some of ours:

- Started as an awesome list on Github in [December
  2015](https://endler.dev/2017/obsolete/)
- Traffic grew continuously. Counting 7.5k stars and over 190 contributors at
  the moment.
- I had the idea to build a website for years now, but my coworker [Jakub]
  joined in May 2020 to finally make it a reality.

{{ figure(src="star-history.png", caption="Github stars over time",
credits="https://star-history.t9t.io") }}

Why did it take almost five years to build a website you ask? Because I thought
the idea was so obvious that others must have tried and failed.

I put it off, even though nobody stepped in to fill this niche.  
I put it off, even though I kept the list up-to-date for five years, just to
learn the tools that are out there.  
You get the idea: When things sound so obvious, it's mostly because they are, so
don't put things off for too long.

## Revenue Model

It took a while to figure out how to financially support the project. We knew
what we didn't want: an SEO landfill backed by adwords. Neither did we want to
"sell user data" to trackers. No, this is exactly what we wanted to fix in the
first place.

We owed all the contributors on Github that the data should stay free for
everyone. Initially we thought about paying from our own pocket, but we'd have
no incentive to maintain the site or extend it with new features.

Github Sponsors was still quite new at that time, but as soon as we realized
that it was an option, it suddenly clicked: Companies that aren't afraid that
users compare their product with their competition have an incentive to support
an open platform that facilitates that. Furthermore, we could avoid bias and
build a product that makes comparison objective and easy.

Sponsoring could be the antidote to aimless growth and rather allow us to build
a lean, sustainable side business. We don't expect analysis-tools.dev to ever be
a full-time job. The market might be too small for that -- and that's fine.

## Tech

Once we had a revenue model, we could focus on the tech. We're both engineers,
which helps iterating quickly.

Initially, I wanted to build something really fancy with
[Yew](https://github.com/yewstack/yew). It's a Rust/Webassembly framework and
your boy [likes Rust/Webassembly](https://endler.dev/2019/tinysearch/)...

I'm glad Jakub suggested something else: GatsbyJS. Now, let me be honest with
you: I couldn't care less about Gatsby. And that's what I said to Jakub: "I
couldn't care less about Gatsby." But that's exactly the point: not being
emotionally attached to something makes me focus on the job and not the tool. I
get stuff done!

From there, it was pretty much easy going: we used a starter template, Jakub
showed me how to use that thing and how the GraphQL integration worked and we
even got to use some Rust! The site runs on Cloudflare as an [edge
worker](https://workers.cloudflare.com/), which built on top of Rust. (Yeah, we
cheated a bit.)

Count 1..2..3 and we were done with a prototype and Bob's our uncle!

## Finding Sponsors

So we had the prototype but zero sponsors so far. What started now was (and
still is) by far the hardest part: convincing people to support us.

We were smart enough not to send cold e-mails, because most companies ignore
them. Rather, we looked at our network and realized that developers reached out
before to add their company's projects to the old static analysis list on
Github.

These were the people we contacted first. We tried to keep the messages short
and personal.

What worked best was a medium-sized e-mail with some context and a reminder that
they contributed to the project before. After that, our plan to start a side
project and a link to our [sponsors
page](https://github.com/sponsors/analysis-tools-dev/).

A prerequisite is that the sponsors page had to be really polished: businesses
look for a reliable partner and a working business proposal.

We received mixed feedback: many people never replied, others passed the message
on to their managers, which never replied, while others again had no interest in
sponsoring an open-source projects in general. That's all fair: people are busy
and sponsorware is quite a new concept.

There was however a rare breed of respondents, which expressed interested, but
needed some guidance. For many, it is the first step towards sponsoring any
developer through Github Sponsors / OpenCollective.

We explained to them that we use OpenCollective as our fiscal host and that they
can get invoices through them since it's a 501(c)(6). If you don't know what
that means, [their docs](https://docs.opencollective.com/help/) explain it quite
clearly.

The task of finding sponsors is never done, but it was very reassuring to hear
from [DeepCode](https://www.deepcode.ai/) - an AI based semantic analysis
service, that they were willing to take a chance on us.

Thanks to them, we could move on to finish the product. It really cannot be
overstated enough: Because of them we can keep the site free for everybody. It
also means free from ads and trackers.

## Marketing

Jakub and me both had own businesses before, but the market dynamics have
changed a lot. It doesn't help to make a thing and buy a couple of Google ads --
you need a marketing plan and an audience.

We decided for a soft launch: deploy the page and let the crawlers index our
site. The page is statically rendered and follows as many SEO guidelines as
possible, so we were slowly seeing some improvements on the search engine
rankings.

After we got some organic traffic and our first votes, we decided to ask our
developer friends to test the page and vote on tools they know and use.

Phase 3 was writing an announcement blog post which, granted, was a bit
click-baity, but it got a few clicks: ["Static Analysis is Broken -- Let's Fix
It!"](https://analysis-tools.dev/blog/static-analysis-is-broken-lets-fix-it)

Phase 4: Writing the announcement tweet on Twitter:

{{ figure(src="tweet.png", caption="We got quite a bit of traffic from that
tweet: 55 retweets, 66 likes and counting",
credits="https://twitter.com/matthiasendler/status/1296162427797671936") }}

Obviously everyone knew that we were asking for support, but we're thankful for
each and everyone that retweeted. This is where a network of like-minded people
can really help.

As soon as we were confident that the site wasn't completely broken, we set off
to announce it on
[Lobste.rs](https://lobste.rs/s/n2ecfs/static_analysis_is_broken_let_s_fix_it)
(2 downvotes),
[/r/SideProject](https://www.reddit.com/r/SideProject/comments/icupeu/we_made_a_website_to_compare_470_static_analysis/)
(3 upvotes) and [HackerNews](https://news.ycombinator.com/item?id=24221708) (144
upvotes, 47 comments). I knew that before, but you need a bunch of luck to get
any attention on social media. It helps to cater the message to the audience and
stay humble.

The feedback from all of that was nuts:

{{ figure(src="traffic.png", caption="Web traffic on launch day",
credits="Cloudflare") }}

Surprisingly, the Cloudflare edge workers didn't break a sweat:

{{ figure(src="worker.png", caption="Edge worker CPU time",
credits="Cloudflare") }}

My boss [Xoan Vilas](https://twitter.com/xo4n) even did a quick performance
analysis and approved:

{{ figure(src="worker.png", caption="Edge worker CPU time",
credits="Cloudflare") }}

High fives all around!

## Now what?

Let's skip the usual marketing pitch: of course we'll add more features, of
course we have more plans for the future. Instead, let's celebrate that
milestone.

Finally, I want you to think about similar niche projects that you've been
putting off for too long. Not anymore! The next success stories is yours. So go
out and make things.

Ah wait, actually before you go would you mind checking out
[analysis-tools.dev](https://analysis-tools.dev/) and hitting that upvote button
for a few tools you can recommend? Hey and if you feel generous today (or you
have a great employer that supports open source), why not check out our
[sponsorship page](https://github.com/sponsors/analysis-tools-dev/)? Hugs! 🤗

[jakub]: https://github.com/jakubsacha
