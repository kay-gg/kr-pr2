# Building
[If you don't want to build, click here!](https://github.com/kyle-gardner/kr-pr2/releases/tag/v1.0.0)

Building is simple. With krpr2 being built with rust, you will of course need rust, and cargo installed. After you install that, in a terminal, enter:

```git clone https://github.com/kyle-gardner/kr-pr2```
```cd ./kr-pr2```
```cargo build --release```

Once you do that, look in the ```target/release``` folder for your executable.
# Using KR-PR2
krpr is a terminal application, so there will be no GUI when you run the exec. 
krpr will take one word as an argument, and it will return how it is pronounced, for example:

doing ```kr-pr2 학년``` will simply return ```항년```
# About KR-PR2
## What is KR-PR2?
KR-PR2 or krpr (Government name: Korean Pronouncer II) is a rewrite of an old program that I wasn't happy with. 
## What does it do?
krpr will take in a single word, and spit out how that word is pronounced. 

The reason that krpr does NOT include a romanization option is because I don't think people that are trying to learn Korean should be exposed to romanized Korean words; only 한글 can truly reflect how a Korean word can be pronounced.
## Why does it do it?
While I was learning Korean, I became frustrated with learning how to pronounce some of the consonant clusters. For example only, I'll use some romanization for anyone who can't read 한글, because I'm sure you don't want to learn an entire alphabet just to figure out what I'm talking about.

Some consonant clusters are hard to pronounce, for example how would you pronounce tb? The Korean language has rules that you should follow when pronouncing consonant clusters, and krpr2 will give you the end result after following those rules.

For example, the word 학년 (Written as Hag-nyeon) is actually pronounced as 항년 (Hang-nyeon). This is an example of Nasalization. There are many more rules than just this, and you can [read about them here](https://en.wikibooks.org/wiki/Korean/Advanced_Pronunciation_Rules)
