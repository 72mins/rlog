rlog is a minimalistic blog/portfolio starter written in Rust, HTML and CSS. If you haven't realized already, it's the site you are currently on.
It is inspired by a similar project, written in Django called [bearblog](https://bearblog.dev/).

# Why not use bearblog

Even though bearblog is an amazing project and arguably better than rlog, I still decided to create my own project to use for my blog posts and as
a personal portfolio.

Bearblog is able to do everything I wanted, but I felt the need to push the limits even further. The main difference between rlog and bearblog is that
bearblog is written in Django (Python), while rlog is written in Rust. The results gained from this are mostly performance and package sizes.

Rust is compiled into a single tiny binary that uses static templates and CSS, which makes it blazingly fast and performant. Not to take away from
bearblog's image, I wanted it to be even faster.


# The design (or lack thereof)

Keeping up with the idea that rlog should be as small, performant and minimal as possible, it uses plain old CSS and keeps the design to the absolute
minimum.

My previous portfolio was the complete opposite; using fancy gradients, animations, a design system, transitions, etc.. I've come to realize
the most important part of a blog and portfolio site is to convey your message. Whoever wants to read my posts or learn more about me, wants to do exactly
that, and not look at animations, wait for loading times and other similar BS.

Even though I have a degree in UI/UX design, this is what I decided to do.

> Return to the original and simple internet. The best design is no design at all.
