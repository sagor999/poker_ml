this is a center piece.
it takes screenshots of cards, etc.

calls relevant programs.
and provides actionable output.


run this from the root folder!


next steps:
need to figure out betting. so need to scan pot size.
can use keras_ocr for it.
BUT. I need to come up with some sort of rules on how to bet based on equity.

Also, another concern is hand equity. Have some lingering doubts about how good of an estimator it is.
It is very logarithmic in nature. Since majority of hands are trash indeed.
Don't know. Don't have good thought about this yet. Just something that lingers in the back of my mind.



split card recognizer into two projects.
one for training.
another one for actual ML work (since I have to do all ML stuff in same container).
so makes sense to put all python scripts into same folder.



ML idea:
run game iteration, but without any betting. 
just based on cards that come. let ML predict if it should fold or call?
or maybe add fixed betting as a loss thing, otherwise ML will keep waiting until all cards to show up.
output from ML should be some sort of hand strength. if should call or fold. 