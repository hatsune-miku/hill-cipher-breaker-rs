# ENGI-9877 Assignment 1 Part B

Student: Zhen Guan (202191382)

---

I wrote a program in Rust to solve this problem. The program (and me) follows the following steps:

## Preprocessing

The program reads the entire file into memory, then remove all line breaks.

## Find most frequent occurring digrams

Iterate through the entire string with step = 2 (The block size), splitting the string into blocks of that size. Count the number of occurrences of each block using a hash map. Sort the map by the number of occurrences, and print the top 20.

### Part of the program output is shown below:

```
("BK", 164), ("GB", 145), ("NF", 75), ("VR", 72), ("NN", 71), ("RP", 68), ("EC", 63), ("QT", 57), ("DT", 57), ("UX", 57), ("EL", 55), ("WO", 52), ("NX", 48), ("JQ", 47), ("CS", 44), ("WR", 43), ("EA", 42), ("ST", 41), ("XP", 41), ("NL", 40)
```

For example, the most frequent digram is `BK`, which occurs 164 times.

## Analysis relation between most frequent digrams and common high frequency digrams

Now we have some most frequent digrams, as well as some common high frequency digrams that given in the question.

### Common high frequency digrams

```
TH, HE, IN, ER, AN
```

It is highly possible that there are some mappings between these digrams and the most frequent digrams. For example, plaintext `TH` could be mapped to ciphertext `BK` (Actually not, but `BK` did mapped to another one of these 5 common digrams).

## Find the encryption key by exhaustive key search

```
for each possible key(a,b,c,d):
    key_matrix = [a, b; c, d]

    for each digram in most frequent digrams:
        ciphertext = (plaintext * key_matrix) mod 26

        if top 10 frequent digrams contains ciphertext:
            print key
```

The program ended up with the only solution:

```
[14, 7; 3, 1]
```

To get the decryption key, we can simply inverse it:

```
[11, 1; 19, 24]
```

## Try decrypt the ciphertext

The decryption is easy with known ciphertext and known decryption key.

The first 200 characters:

```
TRUENERVOUSVERYVERYDREADFULLYNERVOUSIHADBEENANDAMBUTWHYWILLYOUSAYTHATIAMMADTHEDISEASEHADSHARPENEDMYSENSESNOTDESTROYEDNOTDULLEDTHEMABOVEALLWASTHESENSEOFHEARINGACUTEIHEARDALLTHINGSINTHEHEAVENANDINTHEEAR
```

Formatted using `pip wordsegment`:

```
true nervous very very dreadfully nervous i had been an dam but why will you say that i am mad the disease had sharpened my senses not destroyed not dulled them above all was the sense of hearing acute i heard all things in the heaven and in the ear
```

It seems to be a valid English corpus.
