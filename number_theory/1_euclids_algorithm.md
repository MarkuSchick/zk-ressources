# Modular Arithmetic

source: https://crypto.stanford.edu/pbc/notes/numbertheory/euclid.html

### Question: Given three integers a, b, c you can write c in the form?

$$
\begin{aligned}
c = a x + b y
\end{aligned}
$$

### Solution: Solve the related problem

Find greatest common divisor of a and b, i.e. gcd(a,b)
commonly written as (a,b)
When (a, b) = 1, then a and b are coprime
How? Find d that divides a and d divides b. Then we know
that d will divide a - b, where a > b.

We have shrunk the problem and now only have to find gcd(a, a-b)

That is still a hard problem since we have to factorize a and b for that. No one knows how to do it efficiently.

Instead use Euclids algorithm to find gcd(a, b) in a more efficient way:

### Euclids algorithm

#### Simple Example:

gcd(45, 40)

I. 45 = 1 x 40 + 5

thus gcd(45, 40) = gcd(40, 5)

II. 40 = 8 x 5 + 0

Since 40 is a perfect multiple of 5, gcd(45,40) = 5

#### Extended Euclids algorithm

You can find two integers m, n such that

3 = 33m + 27n

rearrange equations so that the remainder are the subjects

$$
\begin{aligned}

    33 &= 1      \times 27 + 6 \\
<=>  6 &= 33 - 1 \times 27     \\
\newline

   27  &= 4      \times 6 + 3 \\
<=>  3 &= 27 - 4 \times 6     \\

\end{aligned}
$$

then substitute I into II.

$$
\begin{aligned}

3 &= 27 - 4 \times  6 \\
  &= 27 - 4 \times (33 - 1 \times 27) \\
  &= (-4) \times 33 + 5 \times 27 \\

\end{aligned}
$$

and we are done:
m = -4 and n = 5

Thus given integers a and b, let d = gcd(a, b) and find integers m, n such that

$ d = ma + nb $

using the extended euclids algorithm.

# Solution

Given integers a,b,c find all integers x, y such that

$c = xa + yb$

1.  Let d = gcd(a, b), and let b = b'd, a = a'd
    Since
    $ xa + yb$ is a multiple of d for any integer x, y,
    solutions only exist when d divides c.
2.  Lets say that $c = kd$. Using the extended Euclidean
    algorith we can find integers m, n such that
    $d = ma + nb$. Thus we have a solution x = km, y=kn
