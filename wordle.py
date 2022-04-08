import nltk
from nltk.corpus import words
nltk.download('words')
letters = ['Q', 'W', 'U', 'I', 'O', 'P', 'A', 'D', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B', 'M']

word_list = words.words()

for first_letter in letters:
    for second_letter in letters:
        word = f'{first_letter}O{second_letter}MA'
        if (word.lower()) in word_list:
            print(word)