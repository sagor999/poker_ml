What is the overall plan for this poker bot?

I have hand ranker. All 2.6M hands are ranked according to their strength.
I have expected value for flop, turn, river in python that needs to be converted into Rust for speed. (since numba fails to compile).

What are the next steps after this?
How do I apply ML to this?
Or do I even need ML for this part?

I like this idea of calculating EV for each possible scenario:
https://www.thepokerbank.com/strategy/mathematics/expected-value/calculate/
Can I write algo that would output that for each possible action and which action should be taken based on that?

Can I run a simulation for the current hand to see what is the most probably outcome will be?
Like, run 1000 or 10,000 different random outcomes to see what is the most probable outcome?
similar concept: https://www.thepokerbank.com/strategy/mathematics/g-bucks/
https://www.thepokerbank.com/strategy/mathematics/hand-combinations/

Chance of opponent folding? Maybe run ML on previous hands to see how often opponent folds when met with a raise???

Some sort of ML or regular algo that estimates that odds of what the other player might potentially have???