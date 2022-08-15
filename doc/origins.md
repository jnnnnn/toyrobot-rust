https://joneaves.wordpress.com/2014/07/21/toy-robot-coding-test/

It may come as a bit of a surprise to many developers in Melbourne (and possibly other places) that I’m the author of the much loved and used “Toy Robot Test”. What may surprise people more is that it’s been around since 2007 and was written when I needed to evaluate a large number of hires for people at ANZ.

It shouldn’t be a huge surprise that it’s not entirely original – lets face it, nothing is. It was heavily influenced by the “Mars Rover” code puzzle that was in use at ThoughtWorks when I was there. I was also involved in hiring people at TW during this period and while I didn’t do this as part of my evaluation, I probably wrote and re-wrote code for this puzzle about 20-30 times as I explored different solutions so I could talk to candidates about it.

When it came to needing one for ANZ, it was all very familiar.

The other part of this that is also very familiar is the one question I keep being asked, which is “Shouldn’t we change it, it’s common and people could ‘cheat’ and copy the solution?”. To this I normally sigh, then start with telling a story all over again. This has happened again recently as one of our wonderful HR recruiting staff asked the same question, so I wrote her an email in response. I was chatting with a good buddy (hey there Travo) who asked me about the code puzzle in a similar way, I showed him the email.

He then suggested that I post my response in a blog. It seems like a good idea, but it’s probably worth understanding how the code puzzle works in my recruitment pipeline. It’s the first filter, which is quite unusual for many people (something else I often get asked about).

Why? Well, the sorts of filters for somebody you want to hire as a software developer are;

-   Are they a competent software developer?
-   Are they an asshole?
-   Can they cope with working with/near me?

In my life experience, people are generally not assholes. So as a result, doing some form of “phone screen” or “cultural interview” first is pointless – because it’s really, really likely they’re a nice person who you’d enjoy hanging out and having a beer/wine/halal drink of choice with. However, if I’m going to hire them as somebody who will write code that’s pretty important. So, I figure – maybe I should ask that. And, in my experience, most people are _fucking terrible_ at writing code (no really, you all are, deal with it).

So, I use the filter that’s going to let the smallest number of potentially non-asshole people through. Then I can chat with them about technical stuff and I can do things like work out exactly how much they do know about technology, if they’re smart and use emacs, and other important things like keybindings and directory structure preferences. Also, doing this gives them a reasonable idea about what it might be like to work with me. So, that kinda sorts out 2 and 3.

Apart from letting some Scala people through this process when even I was too naive to think people would use that borg of a system, it’s worked pretty well. So yeah, the point of this blog post – why I don’t care that the Toy Robot Coding puzzle is well known.

---

Hi Jon,

Can we write a new Robot puzzle please?

Thanks,

(redacted)

---

Hiya (redacted),

I suppose I’m trying to get you to articulate why you think you want to do these things.

You’re telling me things about what is happening (which I already knew about), but there’s another part to this that I want to discuss with you, but I want to understand what you want.

I suspect that you’re worried that people are going to “cheat” the code review and that’s why you want it changed. I’ve had this discussion with everybody since I started doing this – so it’s by no means a new conversation.

Now, this is the very funny part. The general quality of responses from candidates is so poor that they’re too ineffective to even copy a good version of it. Think about that for a minute. Most of the candidates are incapable of copying a working solution to try and “trick” me. What does that say about most programmers? It’s a pretty sad indictment of our industry that they can’t Google a good solution, modify slightly and understand what was written enough to trick me through an interview. It’s embarrassing.

There is also the problem (regardless of what we do) where somebody else can write the code for the candidate.

Now, this is where we have a conversation with the candidate and ask them about the code. Turns out if they didn’t write it – they don’t know how it works, or how to extend it, or how to talk about the potential design issues with it.

Our potential outcomes are as follows;

1. Candidate is able to complete code puzzle honestly and “passes”
1. Candidate is able to complete code puzzle honestly and “fails”
1. Candidate copies from github and passes
1. Candidate copies from github and fails
1. Candidate has submission written by 3rd party and passes
1. Candidate has submission written by 3rd party and fails

Now, our recruitment process will treat 1,3,5 as identical and 2,4,6 as identical during the code review phase. Of course, obvious copying of the wrong names etc may make for great amusement but is otherwise undetectable.

2,4,6 we don’t care about – because they failed anyway

Of 1,3,5 we really need to try and identify 1 separately from 3 and 5, as 1 is the candidate we are at most interested in progressing with.

If we examine the characteristics of 3 and 5, we notice that they are the same. It’s really just “code not written by the candidate”. So, if we choose a different puzzle we are under the incorrect assumption that it will somehow make it “harder” for the candidate to cheat – but option 5 is still untouched.

So, that’s why we talk to the candidate about the code. In depth, and often with reference to the design decisions they have made (or not made) and ask them about how they would extend it. This allows us to detect both 3 and 5.

Having said all of this, I hope you can understand more about why “changing the code puzzle” really doesn’t matter at all. We can change it for other reasons (such as a different problem for the fun of it) but having an “original” problem doesn’t mean anything in terms of improving our hiring decision strategy.
