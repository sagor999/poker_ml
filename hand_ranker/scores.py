import numpy as np
import pandas as pd
import itertools

def build_deck():
    numbers=list(range(2,15))
    suits = ['H','S','C','D']
    deck = []
    for i in numbers:
        for s in suits:
            card = s+str(i)
            deck.append(card)
    return deck

def combinations(arr, n):
    arr = np.asarray(arr)
    t = np.dtype([('', arr.dtype)]*n)
    result = np.fromiter(itertools.combinations(arr, n), t)
    return result.view(arr.dtype).reshape(-1, n)

def check_four_of_a_kind(hand,letters,numbers,rnum,rlet):
    for i in numbers:
            if numbers.count(i) == 4:
                four = i
            elif numbers.count(i) == 1:
                card = i
    score = 700 + four + card/100
    return score

def check_full_house(hand,letters,numbers,rnum,rlet):
    for i in numbers:
        if numbers.count(i) == 3:
            full = i
        elif numbers.count(i) == 2:
            p = i
    score = 600 + full + p/100  
    return score

def check_three_of_a_kind(hand,letters,numbers,rnum,rlet):
    cards = []
    for i in numbers:
        if numbers.count(i) == 3:
            three = i
        else: 
            cards.append(i)
    score = 300 + three + max(cards) + min(cards)/1000
    return score

def check_two_pair(hand,letters,numbers,rnum,rlet):
    pairs = []
    cards = []
    for i in numbers:
        if numbers.count(i) == 2:
            pairs.append(i)
        elif numbers.count(i) == 1:
            cards.append(i)
            cards = sorted(cards,reverse=True)
    score = 200 + max(pairs) + min(pairs)/100 + cards[0]/1000
    return score

def check_pair(hand,letters,numbers,rnum,rlet):    
    pair = []
    cards  = []
    for i in numbers:
        if numbers.count(i) == 2:
            pair.append(i)
        elif numbers.count(i) == 1:    
            cards.append(i)
            cards = sorted(cards,reverse=True)
    score = 100 + pair[0] + cards[0]/100 + cards[1]/1000 + cards[2]/10000
    return score



def score_hand(hand):
    letters = [hand[i][:1] for i in range(5)] # We get the suit for each card in the hand
    numbers = [int(hand[i][1:]) for i in range(5)]  # We get the number for each card in the hand
    rnum = [numbers.count(i) for i in numbers]  # We count repetitions for each number
    rlet = [letters.count(i) for i in letters]  # We count repetitions for each letter
    dif = max(numbers) - min(numbers) # The difference between the greater and smaller number in the hand
    handtype = ''
    score = 0
    if 5 in rlet:
        if numbers ==[10,11,12,13,14]:
            handtype = 'royal_flush'
            score = 900
          # print('this hand is a %s:, with score: %s' % (handtype,score))  I comment the prints so the script runs faster 
        elif dif == 4 and max(rnum) == 1:
            handtype = 'straight_flush'
            score = 800 + max(numbers)
            #print('this hand is a %s:, with score: %s, numbers: %s' % (handtype,score,numbers)) 
        elif 4 in rnum:
            handtype == 'four of a kind'
            score = check_four_of_a_kind(hand,letters,numbers,rnum,rlet)
          # print('this hand is a %s:, with score: %s' % (handtype,score)) 
        elif sorted(rnum) == [2,2,3,3,3]:
            handtype == 'full house'
            score = check_full_house(hand,letters,numbers,rnum,rlet)
          # print('this hand is a %s:, with score: %s' % (handtype,score)) 
        elif 3 in rnum:
            handtype = 'three of a kind'
            score = check_three_of_a_kind(hand,letters,numbers,rnum,rlet)
          # print('this hand is a %s:, with score: %s' % (handtype,score)) 
        elif rnum.count(2) == 4:
            handtype = 'two pair'
            score = check_two_pair(hand,letters,numbers,rnum,rlet)
          # print('this hand is a %s:, with score: %s' % (handtype,score)) 
        elif rnum.count(2) == 2:
            handtype = 'pair'
            score = check_pair(hand,letters,numbers,rnum,rlet)
          # print('this hand is a %s:, with score: %s' % (handtype,score)) 
        else:
            handtype = 'flush'
            score = 500 + max(numbers)/100
          # print('this hand is a %s:, with score: %s' % (handtype,score)) 
    elif 4 in rnum:
        handtype = 'four of a kind'
        score = check_four_of_a_kind(hand,letters,numbers,rnum,rlet)
      # print('this hand is a %s:, with score: %s' % (handtype,score)) 
    elif sorted(rnum) == [2,2,3,3,3]:
       handtype = 'full house'
       score = check_full_house(hand,letters,numbers,rnum,rlet)
     # print('this hand is a %s:, with score: %s' % (handtype,score)) 
    elif 3 in rnum:
        handtype = 'three of a kind' 
        score = check_three_of_a_kind(hand,letters,numbers,rnum,rlet)
      # print('this hand is a %s:, with score: %s' % (handtype,score)) 
    elif rnum.count(2) == 4:
        handtype = 'two pair'
        score = check_two_pair(hand,letters,numbers,rnum,rlet)
      # print('this hand is a %s:, with score: %s' % (handtype,score)) 
    elif rnum.count(2) == 2:
        handtype = 'pair'
        score = check_pair(hand,letters,numbers,rnum,rlet)
      # print('this hand is a %s:, with score: %s' % (handtype,score)) 
    elif dif == 4:
        handtype = 'straight'
        score = 400 + max(numbers)
      # print('this hand is a %s:, with score: %s' % (handtype,score)) 

    else:
        handtype= 'high card'
        n = sorted(numbers,reverse=True)
        score = n[0] + n[1]/100 + n[2]/1000 + n[3]/10000 + n[4]/100000
      # print('this hand is a %s:, with score: %s' % (handtype,score)) 
        
    return score
    



def handvalues(combinations):
    scores =[{"hand": i, "value": score_hand(i)} for i in combi] # We iterate over all combinations scoring them
    scores = sorted(scores, key = lambda k: k['value']) # We sort hands by score
    return scores

    
deck = build_deck() # We create our deck
combi = combinations(deck,5) # We create an array containing all possible 5 cards combinations
hand_values = handvalues(combi)
x = [i.get("hand","") for i in hand_values] # making a list of hands
y = [i.get("value","") for i in hand_values] #making a list of values

data = {'hands':x, 'value':y} # making a dictionary of hands and values
df = pd.DataFrame(data) # making a pandas dataframe with hands and values
df.to_csv("./hands.csv", sep=",", header=False, index=False)
# print(df)