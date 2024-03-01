# tempo-edf-raspberry-pi-led

A simple multi langage raspberry pi set of tools to run on you raspberry pi to show a RGB led color of the day

## Why

Because on red days we pay 3x the classical pricing so we need to know it by showing a red light

## Project

This project was a POC and work with a :

- Rasperry pi 3 A+ model
- [A kit of many things](https://www.amazon.fr/dp/B0BJF6TZJX?psc=1&ref=ppx_yo2ov_dt_b_product_details) (gpio board / wire / RGB led / a resitance of 320Ω)

## Langages

I wanted to have this project in multi lang

- Typescript because that's my favorite langage
- Rust because i love to hate him
- Go, to compare simplicity with Rust
- Zig, really want to try it, due to bun use it and retro-compatibility with C/C++
- C++ because after working on Rust, I think this old one is better

  If you see thing badly wrote, don't hesitate to PR or issue me!

## Schema

`raspi-gpio get` Will help you find the right entry but this can help you too:

**⚠️ depending on you os install, on my side I sometimes encontered 530/529 pin entry instead of 17/27 for eg, you need to check via command etc..**

https://www.freva.com/wp-content/uploads/2018/08/PI_GPIO_HEADER.webp

- I used a **brown** wire for ground connected to N°6 entry (the longest one on RGB)
- I used a **red** wire connected to the red (the one alone after the longest RGB) **GPIO 4**  (which is pin entry N°**7**)
- I used a **green** wire connected to **GPIO 17** (which is pin entry **11**)
- The latest one is for the **blue* **GPIO 27** (which is pin entry **13**)


### Typescript

#### Install

- `cd typescript && npm install && cp .env.example .env`
- adapt your .env regarding the informations you find on you enty if needed

#### Start

- `npx tsx main.ts` will compile and run your code, will show you the right color for the led about the tempo DAY 😁🎉
  


