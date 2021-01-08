# pop-prob
`pop-prob` is a population probability calculator. It can perform the following:
- Calculate the most likely population size from a sample from this population (with replacement) and the number of unique elements within this sample
- Calculate the probability that a population is a given size from a sample from this population (with replacement) and the number of unique elements within this sample
- Calculate the likelihood of selecting a given number of unique elements from a sample from a population of a given size

## Inspiration
I was playing an online trivia game one day. After 40 questions, the next question was a repeat. 10 questions later, another repeat. This made me think that the question bank must be relatively small. But how small? In all, I went through 80 questions, 5 of which were repeats. I wanted to know the most likely size of the word bank from this information, as well as the likelihood that this size was correct. However, I could not find an adequate answer to this problem online. So I created one. Now I know the question bank most likely contained 605 words (there is a 0.13% chance this is correct).

## Usage
Add `pop-prob` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
pop-prob = "0.1"
```

### CLI
`pop-prob` is a Rust library. A CLI tool for `pop-prob` can be found [here](https://github.com/beamer159/pop-prob-cli).
