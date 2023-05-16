# Division

source: https://crypto.stanford.edu/pbc/notes/numbertheory/division.html

We proofed in [modular arithmetic](./0_modular_arithmetic.md) that we can only divide by y iff the inverse $y^{-1}$ of y exists.

We shall now show why this is the case.

We wish to find all z such that $yz=x (\text{mod } n)$, which by definition means

$$
\begin{aligned}

x &= zy + kn \\

\end{aligned}
$$

for some integer k.

But from [euclids algorithm](./1_euclids_algorithm.md) we know that $d = gcd(y, n) $.

tbd.

# Computing inverses

### Example: does $$7^{-1} (\text{mod } 19)$$ exist and if so what is it?

### Solution

1. A inverse exists iff $gcd(y, n) = 1$.

$$
\begin{aligned}

19 &= 2 \times 7 + 5 \\
7  &= 1 \times 5 + 2 \\
5  &= 2 \times 2 + 1 \\

\end{aligned}
$$

So $gcd(7, 19) = 1$

2. Compute $y^{-1}$ by using [extended euclids algorithm](./1_euclids_algorithm.md)

Rearrange above equations to find the inverse by moving the remainder as a subject, while using the remainder as the subjec.t
Substituting the second in the last equation leads to.

$$
\begin{aligned}

1  &= 5 - 2 \times 2 \\
   &= 5 - 2 \times (7 - 1 \times 5) \\
   &= 3 \times 5 - 2 \times 7 \\

\end{aligned}
$$

Substituting the first in the equation leads to.

$$
\begin{aligned}

1  &= 3 \times 5 - 2 \times 7 \\
   &= 3 \times (19 - 2 \times 7) - 2 \times 7 \\
   &= - 8 \times 7 + 3 \times 19   \\
\end{aligned}
$$

We can see that in modulo 19 the inverse of 7 is
-8, since $1 = -8 \times 7 (\text{mod } 19) $.
