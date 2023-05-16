course-2023-01-20 Session 8 Notes

# Exercises

---

## Question

Suppose that `x mod 30 = 2`

What is obtained about `x mod 2`, `x mod 3`, `x mod 5` and `x mod 7`?

## Answer

From `Lemma 1` we know that

If a | b and b | c then a | c

, where a | b when "a divides b".

We know the following about

### x mod 2

Since 30 divides (x - 2), and 2 divides 30 we know that

$$
\begin{aligned}

x - 2 \bmod  2 = 0 \\

x \bmod  2 = 0

\end{aligned}
$$

### x mod 3

$$
\begin{aligned}

x \bmod  3 = 2 \\

\end{aligned}
$$

### x mod 5

$$
\begin{aligned}

x \bmod  5 = 2 \\

\end{aligned}
$$

### x mod 7

Nothing?

---

## Question

You're Eve, intercepting a message from Alice to Bob. Alice asked Bob to choose a prime larger than 30 to construct a prime field. You choose
p = 31, g = 2

How many unique choices of exponent x in $ g ^x = a \pmod p $
does Alice now have? A unique choice is any uniquely obtainable values for
a.

For instance, $2^{17} = 4$ and $2 ^ {32} = 4$ are not unique.

## Answer

---

We call the smallest positive value for which $g^q = 1 \pmod p$ the order of g. Also called the number of elements.

Lagrange's theorem states that for any finite group G, the order (number of elements) of every subgroup of G divides the order of G.

Explanation: https://www.youtube.com/watch?v=TCcSZEL_3CQ

`That means for the group G = {1, ...., 30}, any subgroup g of G has an order weakly smaller than G.`

---

First observe that p - 1 = 30. Lagrange's Theorem states that the order of g divides 30, i.e. G = {1, ..., 30}.

The integers dividing 30 are:
1, 2, 3, 5, 6, 10, 15, 30.

So we have possible subgroups of order
2, 3, 5, 6, 10, 15

That does not mean these subgroups have to exist!

For instance

$$
\begin{aligned}

31 &= 0 \pmod{31} \\
31 - 1 &= -1 \pmod{31 } \\

(31 - 1) ^ 2 &= (-1)^2 = 1 \pmod{31} \\


\end{aligned}
$$

or

$$
\begin{aligned}

(31 - 2) ^ 2 &= (-2)^2  \pmod{31} \\


\end{aligned}
$$

, so the element 29 has the order 2.

See:

$$
\begin{aligned}

2^{1} \bmod  29 = 2 \\

2^{2} \bmod  29 = 4 \\

2^{3} \bmod  29 = 8 \\

2^{4} \bmod  29 = 16 \\

2^{5} \bmod  29 = 3 \\

2^{6} \bmod  29 = 5 \\

2^{7} \bmod  29 = 5 \\

\end{aligned}
$$

### Alternatively we can see that:

For g = 2 and p = 31 we have the following subgroup.

2, 4, 8, 16, 1 (32 = 31 + 1), 2 (64 = 2\* 31 + 2), ...
So the element 2 has the order 5.
(2, 4, 8, 16, 1).
There are 5 unique choices for Alice's private key. If Alice wants to prevent this attack, she must check that the generator does not lie in a small cycle.

---

# Question

Bob would have chosen p = 83, g = 2.
How many unique choices for x
does Alice have now?

---

g = 2, p = 83, how many unique x are they?

Unique x are defined by all x such that a is a unique value.

$ g ^x = a \pmod p $

Since p - 1 = 82 we have the possible unique choices:

2, 41.

---

What if Bob chose p = 83, g = 3?

Since p - 1 = 82, it must be (p - 1) / 2 = 41, where 42 - 1 = 41
So we only have order 41. that makes g = 2 and p = 8 a small group which can be dangerous.
