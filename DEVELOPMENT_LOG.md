# Development Log

As mentioned in the `README.md`, this document is my personal journal for this
project. How I am feeling, what I am focused on, decision I make and why, and
whatever else I feel like deserves capturing. These are personal notes that I
am making public, rather than a polished document that I think others will be
able to get information out of it.

That being said, maybe it can inspire someone else who is afraid to commit to a
huge project they are severely underqualified for but can learn a ton from.

This project will probably become my life's work in software. I hope to look back
on this document one day and be proud of everything I have worked on here.

This is going to be a long journey...

## Preface

Hello, world!

### Why Build an OS from Scratch?

I guess I should probably start by answering that question, so that when I
inevitably find myself spending hours debugging something I don't lose
motivation and give up.

To be completely honest, I'm not completely sure why I want to do this. I don't
even really *want* to. It is more of a compulsion than a conscious decision or a
desire. Code is never in a bubble. It is always relying on or working with
software that someone else wrote. Whether it is someone I am working with, or an
external library my code depends on, I often find myself digging through other
code, trying to figure out how it works, and questioning "Why would someone ever
do that this way?". Usually, it is because they thought of something I didn't,
and until I really sit down and think through the problem, I won't truly
understand why. While it is amazing that we live in a world where people freely
share their software for others to use and build on, I have a selfish desire for
control and awareness of every bit of the system that often gets in the way of
real progress. I need to satisfy this itch somehow.

All of my best (and most enjoyable) learning has been through taking problems
with well known solutions, and implementing them myself. From my Data Structures
to Distributed Systems classes, to eventually my work, I found solving real
problems to be the most fun part of software, whether someone else had solved.
the problem before or not.

Building an operating system from scratch is the epitome of all of this. It is
as standalone as it gets (without making custom hardware, of course). I have
free reign to design the system however I want. When things break, I have no one
to blame but myself. But, when things work, I can say that I did it all. I don't
feel like I have a lot of creativity in the traditional sense, but when I am
really thinking through hard problems, I feel it come through. If there is a
path forward, I will find it if I approach the problem from enough angles.

To put it as succinctly as possible:

I love solving problems, challenging myself, and being able to look back and see
how much progress I have made. Working with software is inherently standing on
the shoulders of giants. I'd like to walk in their footsteps instead for once,
and maybe one day, I'll catch up.

### My Background

I suppose I might as well paint the full picture if I truly want to spend a huge
chunk of my life working on this.

#### Early Interest

In elementary school, my dad handed me a book that was meant to teach kids how
to code in Python. I don't remember why, but I'd imagine that I probably wanted
to know what he did for work (obviously software). I followed the little lessons
in the book, but didn't really learn much. I mostly copied what was there and
was proud to have made a little skiing game. After trying to make changes and
put my own spin on it, I realized that I had no idea what was going on, and gave
up pretty quickly.

Then, some point in middle school, I decided to do a Khan Academy course on
programming in JavaScript. This time, I actually learned from it, and really
made some good progress. Again, I don't remember the spark that made me choose
to do this, but I remember I really enjoyed writing animated color gradients on
the little canvas they provided. I remember getting to the lesson on prototypes
and giving up. But, this was where I really started learning how to code.

The next interaction with code I remember doing was sophomore year of high
school, when one of my classmates made a little racing game website. A bunch of
us would try to get the fastest lap time, since there was a little leaderboard.
Not to brag, but I was pretty good at it, I was consistently in the top 3.
Eventually, I got bored of playing the game itself, and decided to try poking
around in the code. I figured out I could use the chrome developer console to
change some variables to make the game easier. I tweaked some values to make the
steering more responsive and the max speed higher. After destroying the
leaderboard, I showed my classmate that made it, and spent my time on other
things.

#### The Pivot

All through high school, I knew I wanted to go to college for either science or
engineering. By the time I was applying, I had narrowed it down to either
Chemistry or Chemical Engineering. I took AP Chemistry my Junior year and loved
it - I had an amazingly overqualified teacher who really drove us to work to our
potential. In hindsight, I probably would have liked Physics just as much, but
the way my class schedule ended up, I didn't take that until my senior year, and
it was mind-numbingly easy for me. We spent the whole year on kinematics, which
was covered in two classes of college physics when I took it.

The seed of doubt of whether I wanted to follow my planned chemistry track
entered my mind when school shut down for COVID in the middle of my Junior year.
I realized two things:

 1. If I pursued Chemistry, most of my time would be spent doing math or
    reading papers (ironic, in hindsight). I would only truly be enjoying
    myself when I was in the lab, and I knew that even that would stop being
    interesting to me eventually.
 2. I would never be able to fully invest myself in Chemistry. I knew that
    whatever I did, I would want to be the best I possibly could at it. That
    means I would need to be able to spend as much time as possible doing it.
    That requires a lab and funding. Two things I knew would be difficult to
    come by, especially during the pandemic.

I realized that switching to a programming path (I really didn't know what
Computer Science as a field was yet) could address both of these problems, but I
also knew that until I really committed to trying it, I wouldn't know if I would
be happy devoting my life to it.

So, for my capstone project senior year, I chose to do a programming project. I
challenged myself to make a Tic-Tac-Toe bot. Looking back now, it was such a
small project, but I really had to re-learn how to program. After implementing
the game, I iterated on opponent strategies, starting with hard coded responses,
then scoring different states, until I eventually stumbled into recursion and
the minimax algorithm. Of course when I was doing this, I had no idea the
algorithm had a name, I had intentionally kept myself from looking up strategies
because I wanted to figure it out myself.

After my bot was perfect and unbeatable, I remember having my parents play it,
and sending it to my friends, and thinking it was the coolest thing ever, that I
had taught a computer to play a game perfectly.

This was the moment I decided to switch my major to Computer Science.

#### College

I ended up going to Rensselaer Polytechnic Institute, since out of the chemistry
programs I applied to it was the best computer science program for the cost. I
had never taken a computer science class before, but I quickly got into a rhythm
and did very well. I loved Data Structures, Computer Organization, Operating
Systems, and Distributed Systems - all of the low level classes. I graduated in
3 years and stayed for my masters in 1.5. Not too much is of note here. I did a
lot of tutoring / mentoring / TAing, which kept me busy. I also love teaching.

The only notable programming I did outside of school was MIT's yearly Battlecode
competition, I competed every year since 2022. I cannot recommend this enough to
anyone and everyone interested in software.

#### Work

After my second year at school, I started as an intern at Nasuni, a company that
develops a Network Attached Storage product that uses a cloud storage backend as
the primary copy. My role was in the Datapath team, so I got to work in the
nitty gritty details of the core of the product. I have been working there for
almost 3 years now (at the time of writing this). I am very glad this was the
internship I chose, I have learned so much working there, and my managers and
team have given me the freedom and tools to learn and grow.

### That's All I Have To Say about That

That takes us to now, when I am starting this project. This will definitely take
years of work. I'm not sure how much time I will dedicate to it, but I hope to
consistently work on it and eventually have something worth looking at.

## 2026-3-10

### Day 1 Progress

I did a lot of reading to make a simple "Hello, World!" program, and made the
key design decision to use fat pointers. Mainly because my experience with
getting modern C++ to interact with legacy C APIs is horrible. As an example
since `std::string_view` doesn't guarantee null termination, instead of passing
substrings you have to make a full copy of a string that you want to pass.

The other main reason I chose this was because I wanted my OS to be different.
This means I have new problems to address that a lot of people following guides
wouldn't have to worry about. I don't want to follow a blueprint for this, that
defeats the whole purpose, I might as well use the linux kernel, or an existing
distribution. At that point, maybe I'll even try windows...

All jokes aside, day 1 was successful. I got something to run.

### Next Steps

I need to make the jump from 16-bit real mode to 32-bit protected mode. Then,
eventually, I'll get into 64-bit long mode and to the kernel.

