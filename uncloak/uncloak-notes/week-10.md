# An Introduction to Mathematical Cryptography

## Chapter 1.2 Divisibility and greatest common divisors

`Deﬁnition`

Let a and b be integers with $b \ne 0$. We say that b divides a, or that a is divisible by b, if there is an integer c such that a = bc.

We write $b \mid a$ to indicate that b divides a. If b does not divide a then we write $b \nmid a$.

_integers are divisible when their inverse exists_

`Definition`

A _common divisor_ of two integers a and b is a positive integer d that divides both of them. The `greatest common divisor` of a and b is, as its name suggests, the largest positive integer d such that $d \mid a$ and $d \mid b$. The greatest common divisor of a and b is denoted gcd(a, b). If there is no possibility of confusion, it is also sometimes denoted by (a, b).

`Deﬁnition. (Division Algorithm)` Let a and b be positive integers. Then a divided by b has quotient q and remainder r means that

$$ \begin{aligned}

a &= b \cdot q + r \text{, with } 0 ≤ r < b \

\end{aligned} $$

Here, the values of q and r are _uniquely_ defined by a and b.

If d is any common divisor of a and b, then it is clear from above equation that d is also a divisor of r. Similarly, if e is a common divisor of b and r, then (1.1) shows that e is a divisor of a. In other words, the common divisors of a and b are the same as the common divisors of b and r;

$$ \begin{aligned}

gcd(a, b) = gcd(b, r)

\end{aligned} $$

We can continue this process as

$$ \begin{aligned}

b &= r \cdot q' + r' \text{, with } 0 ≤ r' < b' \

\end{aligned} $$

or

$$ \begin{aligned}

gcd(b, r) &= gcd(r, r')

\end{aligned} $$

Now continuing this process until $gcd(s,0) =s $ is the `Euclidean Algorithm`.

### Example: Calculate gcd(2024, 748)

$$ \begin{aligned} 2024 &= 748 \cdot 2 + 528 \ 748 &= 528 \cdot 1 + 220 \ 528 &= 220 \cdot 2 + 88 \ 220 &= 88 \cdot 2 + 44 \ 88 &= 44 \cdot 2 + 0

\end{aligned} $$

Hence, $gcd(2024, 748) = 44$

From the ouptut of the _Euclidean Algorithm_, we can make an interesting observation. Rewriting the first line with a = 2024 and b = 748:

$528 = a - 2b $

And substitute the result in line 2

$b = (a - 2b) + 220 <=> 220 = -a + 3b$

And substituting both result in line 3

$528 = 220 \cdot 2 + 88$

$a - 2b = (-a + 3b) \cdot 2 + 88 <=> 88 = 3a - 8b $

Finally we subsitute

$220 = −a + 3b$ and $88 = 3a − 8b$

in the last line

$−a + 3b = (3a − 8b) · 2 + 44$, so $44 = −7a + 19b$

In other words $−7 · 2024 + 19 · 748 = 44 = gcd(2024, 748)$

We can write the gcd of two numbers a and b as the linear combination of said two numbers!

`In general, it is always possible to write gcd(a, b) as an integer linear combination of a and b, a simple sounding result with many important consequences.`

`Theorem 1.11 (Extended Euclidean Algorithm)`. Let a and b be positive integers. Then the equation

$au + bv = gcd(a, b)$

always has a solution in integers u and v.

`Deﬁnition` Let a and b be integers. We say that a and b are relatively prime if gcd(a, b) = 1.

Generally, any equation of the form

$Au + Bv = gcd(A, B)$

can be reduced to a case of relative prime numbers by dividing both sides by gcd(A,B)

$\frac{A}{gcd(A,B)}u + \frac{B}{gcd(A,B)}v = 1$

Now the numbers $a = A/ gcd(A, B)$ and $b = B/ gcd(A, B)$ are relatively prime and satisfy $au+bv = 1$.

Continuing with the example above

$−7 · 2024 + 19 · 748 = 44$

and dividing both sides by $gcd(2024, 748) = 44$ we get

$-7 \cdot 46 + 19 \cdot 17 = 1$

Thus $2024/44 = 46$ and $748/44 = 17$ are relatively prime, and $u = −7$ and $v = 19$ are the coeﬃcients of a linear combination of 46 and 17 that equals 1.

## Chapter 1.3 Modular arithmetic

`Deﬁnition` Let $m \geq 1$ be an integer. We say that the integers a and b are congruent modulo m if their diﬀerence a − b is divisible by m. We write $a \equiv b \pmod m $

`Proposition 1.13`. Let m ≥ 1 be an integer.

(a) If $a1 \equiv a2 \pmod m$ and $b1 \equiv b2 \pmod m$, then

$a1 ± b1 \equiv a2 ± b2 \pmod m$

and

$a1 · b1 \equiv a2 · b2 \pmod m$.

(b) Let a be an integer. $a · b \equiv 1 \pmod m$

Then for some integer b if and only if gcd(a, m) = 1\. If such an integer b exists, then we say that b is the (multiplicative) inverse of a modulo m. (We say "the" inverse, rather than "an" inverse, because any two inverses are congruent modulo m.)

In this case b is the inverse of a modulo m.

`Deﬁnition`. Proposition 1.13(b) tells us that a has an inverse modulo m if and only if gcd(a, m) = 1\. Numbers that have inverses are called units. We denote the set of all units by

$$ \begin{aligned} (Z/mZ)^* &= {a ∈ Z/mZ : gcd(a, m) = 1} \ &= {a ∈ Z/mZ : \text{a has an inverse modulo m}}.

\end{aligned} $$

The set (Z/mZ)∗ is called the group of units modulo m.

`Deﬁnition`. Euler's phi function (also sometimes known as Euler's totient function) is the function φ(m) deﬁned by the rule

φ(m) = # (Z/mZ) = #{0 ≤ a < m : gcd(a, m) = 1}

### 1.3.2 The fast powering algorithm

Example 1.19\. Suppose that we want to compute $3^{218} \pmod{1000}$.

A naive approach would just calculate

g1 = 3; g2 = 3 * 3, .... etc.

That is very slow hower. The fast powering algorithm instead writes 218 as a sum of powers of 2,

$218 = 2 + 2^3 + 2^4 + 2^6 + 2^7 $

and now we write

$3^{218} = 3^{2+2³ +2⁴ +2^5 +2⁶} = 3^2 · 3^{2^3} · 3^{2^4} · 3^{2^5} · 3^{2^6}$

since each number is the _square_ of the preceeding number it is relative simple to compute the series of values. We further take the modulo 1000 each step so that we only need do store 3 digits

## 1.4 Prime numbers, unique factorization, and ﬁnite ﬁelds

`Deﬁnition`. An integer p is called a prime if p ≥ 2 and if the only positive integers dividing p are 1 and p

`Theorem 1.21 (The Fundamental Theorem of Arithmetic)`. Let a ≥ 2 be an integer. Then a can be factored as a product of prime numbers $a = p_1^{e1} · p_2^{e2} · p_3^{e3} · · · p_4^{e4}$ .

`Deﬁnition`. The fundamental theorem of arithmetic (Theorem 1.21) says that in the factorization of a positive integer a into primes, each prime p appears to a particular power. We denote this power by ordp (a) and call it the order (or exponent) of p in a. (For convenience, we set ordp (1) = 0 for all primes.)

For example, the factorization of 1728 is $1728 = 26 · 33$ , so

$ord_2 (1728) = 6$, $ord_3 (1728) = 3$, and $ord_p (1728) = 0$ for all primes p ≥ 5.

`Proposition 1.22` Let p be a prime. Then every nonzero element a in Z/pZ has a multiplicative inverse, that is, there is a number b satisfying

$ab \equiv 1 \pmod m$

We denote this value of b by a−1 mod p, or if p has already been speciﬁed, then simply by a−1 .

`Deﬁnition` If p is prime, then the set Z/pZ of integers modulo p with its addition, subtraction, multiplication, and division rules is an example of a ﬁeld. If you have studied abstract algebra (or see Section 2.10), you know that a ﬁeld is the general name for a (commutative) ring in which every nonzero element has a multiplicative inverse. You are already familiar with some other ﬁelds, for example the ﬁeld of real numbers R, the ﬁeld of rational numbers (fractions) Q, and the ﬁeld of complex numbers C.

# 1.5 Powers and primitive roots in ﬁnite ﬁelds

`Theorem 1.25 (Fermat’s Little Theorem).` Let p be a prime number and let a be any integer. Then

$ a^{p−1} \equiv 1 \pmod p \text{ if } p\nmid a \ a^{p−1} \equiv 0 \pmod p \text{ if } p\mid a \ $

In other words with a prime p (i.e 5) we know that $a^4 = 1 \pmod 5$

`Fermat's Little Theorem` is very helpful in trying to find the inverse of some number as:

$a^{−1} \equiv a^{p−2} \pmod p$, since

$a^{p-2} * a = a^{p-1} = 1 \pmod p$

`Example 1.29`. Consider the number m = 15485207\. Using the powering al- gorithm, it is not hard to compute (on a computer) $2^{m−1} = 215485206 ≡ 4136685 \pmod {15485207}$

We did not get the value 1, so it seems that Fermat's little theorem is not true for m. What does that tell us? If m were prime, then Fermat's little theorem says that we would have obtained 1\. Hence the fact that we did not get 1 proves that the number m = 15485207 is _not prime_. Wow!

`Proposition 1.30`. Let p be a prime and let a be an integer not divisible by p. Suppose that

$a^n \equiv 1 \pmod p$ Then the order of a modulo p divides n. In particular, the order of a divides p − 1.

We deﬁne the order of a modulo p to be the smallest exponent k ≥ 1 such that

$a^k \equiv 1 \pmod p$.

`Theorem 1.31 (Primitive Root Theorem)`. Let p be a prime number. Then there exists an element $g ∈ F^∗_p$ whose powers give every element of F∗p , i.e.,

$F^*_p = {1, g, g^2 , g^3 , . . . , g^{p−2} }.$

Elements with this property are called primitive roots of Fp or generators of F∗p . They are the elements of F∗p having order p − 1.
