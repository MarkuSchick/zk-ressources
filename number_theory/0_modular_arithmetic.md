# Modular Arithmetic

source: https://crypto.stanford.edu/pbc/notes/numbertheory/arith.html

$$
\begin{aligned}

n &:= \text{positive integer} \\

\newline

\mathbb{Z_n} &:= \text{set of n} \\
             & =  [0 ... n -1 ] \\
             &:= \text{called residues} \\


\newline
x, y &:= \text{integers} \\



\newline
x = y \text{ ( mod n )} &:= \text{ x and y are the same when x and y differ by a multiple of n} \\
&:= \text{ x and y are congruent modulo n} \\

\newline

\end{aligned}
$$

`We might emit n when clear from context`

- Each integer x is congruent ot some y in $\mathbb{Z_n}$
- when we substract multiples of n from x to each y in $\mathbb{Z_n}$, we say we are reducing x modulo n, and is the residue

- elements in $\mathbb{Z_n}$ are called residues

#### Examples

$$
\begin{aligned}

3 = 38 \text{ ( mod 5 )} \\
\text{, since } 38 = 5*7 + 3 \\

\newline
-3 = 11 \text{ ( mod 14 )} \\
\text{, since } -3 = -1 * 14 + 11 \\


\end{aligned}
$$

### Question: How can we do arithmetic in $\mathbb{Z_n}$?

### Answer: Given two elements x and y in $\mathbb{Z_n}$, we can add, subtract, multiply, and divide them as integers, and the result will be congruent to one of the elements in $\mathbb{Z_n}$

#### Addition && Multiplication && Substraction

$$
\begin{aligned}

6 + 7 = 1 \text{ ( mod  12)} \\
3 * 20 = 10 \text{ ( mod  50)} \\
12 - 14 = 16 \text{ ( mod  18)} \\


\end{aligned}
$$

#### And what about division?

What is a division inuitively? A division should "undo multiplication".
Problem is that there can be different candidates.

$$
\begin{aligned}

5 * \bold{2} = 4 \text{ ( mod  6)} \\
2 * \bold{2} = 4 \text{ ( mod  6)} \\

\end{aligned}
$$

Both 5 and 2 multiplied by 2 are congruent to 4 in $\mathbb{Z_6}$.

Solution:
We require `uniqueness` of the division operation.
i.e. x divided by y is modulo n is only defined when there is a unique z in $\mathbb{Z_n}$ such that x = y \* z.

We obtain the solution as follows.
Suppose that there are two z ($z_1, z_2$) s.t.

$z_1y=z_2y (\text{mod }n)$

then for some k we have from the definition of the modulo operator ($z_1y = z_2y + kn $):

$$
\begin{aligned}

y(z_1 - z_2) &= kn

\end{aligned}
$$

Let d be the greatest common divisor (gcd) of n and y then.
Then n/d divides $z_1 - z_2$ since it cannot divide y, thus we have

$$
\begin{aligned}
z_1y &= z_2y (\text{mod }n) \\
\text{iff} \\

z_1 - z_2 &= kn/d  \\
z_1       &= z_2 + k * n/d \\
z_1       &= z_2 (\text{mod }n/d)
\end{aligned}
$$

#### Inverses

There is only a unique z iff it there exists a $w \in \mathbb{Z_n} $ s.t. $yw=1 (\text{mod }n)$

Proof by contradiction:
Suppose there is another w' s.t. $yw'=1 (\text{mod }n)$
Then $yw' = yw (\text{mod }n)$
and $wyw =wyw'$, which implies $w=w'$, which is a contradiction.
