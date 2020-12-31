

split card recognizer into two projects.
one for training.
another one for actual ML work (since I have to do all ML stuff in same container).
so makes sense to put all python scripts into same folder.



ML idea:
run game iteration, but without any betting. 
just based on cards that come. let ML predict if it should fold or call?
or maybe add fixed betting as a loss thing, otherwise ML will keep waiting until all cards to show up.
output from ML should be some sort of hand strength. if should call or fold. 


need to add support for detecting the button, to provide info on position.


ML and simulated data collected from 6 players per table.
ML idea for training, which I think is actually doable. Not sure about results though.

input:
game state: pre-flop, flop, turn, river.
my hand: two inputs. 1-52 for cards. 0, means no card.
community cards depending on that state: 
  pre flop: 0 0 0 0 0
  flop:     x x x 0 0
  turn:     x x x x 0
  river:    x x x x x
hand win rate: 0-1, from simulated hands data.
hand was winning on flop: 0-1, from simulated hands data.
hand was winning on turn: 0-1
hand was winning on river: 0-1
hands equity: based on projected opponents range. for preflop it will use starting hand data.

output: predict if should flop or call.

ground truth: generated based on the actual cards dealt to all players during dataset generation.

I think I will generate 10mil games. Each game is 4 states, so 40mil data points.

And see if it can get better then say 50-60% accuracy. who knows.
though, frankly speaking I have doubts due to random nature of cards. 
at best it will predict that there are some hands that always have a higher chance of winning (like AA).

this doesn't include any type of betting, since I am not sure how to simulate that.
