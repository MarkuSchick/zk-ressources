# Chapter 4.3: Probability theory

# 4.3.1: Basic concepts of probability theory

`Deﬁnition`. A sample space (or set of outcomes) is a ﬁnite set Ω. Each outcome ω ∈ Ω is assigned a probability Pr(ω), where we require that the probability function Pr : Ω → R satisfy the following two properties:

_4.14_

(a) 0 ≤ Pr(ω) ≤ 1 for all ω ∈ Ω and

(b) $\sum_{ω ∈ Ω} Pr(ω) = 1$

`Deﬁnition` An event is any subset of Ω. We assign a probability to an event E ⊂ Ω by setting

_4.15_

$Pr(E) = \sum_{ω ∈ E} Pr(E)$

In particular, Pr(∅) = 0 by convention, and Pr(Ω) = 1 from (4.14)(b).

`Deﬁnition`. We say that two events E and F are disjoint if E ∩ F = ∅. It is clear that

$Pr(E ∪ F ) = Pr(E) + Pr(F )$ if E and F are disjoint,

When E and F are not disjoint, the probability of the event E ∪ F is not the sum of Pr(E) and Pr(F )

_4.16_

$Pr(E ∪ F ) = Pr(E) + Pr(F ) − Pr(E ∩ F )$

`Deﬁnition` The complement of an event E is the event E c consisting of all outcomes that are not in E, i.e.,

$ E^c = \set{ω ∈ Ω : ω \not ∈ E } $

The probability of the complementary event is given by

_4.17_

$Pr(E^c) = 1 − Pr(E)$

`Deﬁnition`. Two events E and F are said to be independent if

$Pr(E ∩ F ) = Pr(E) · Pr(F )$

# 4.3.2: Bayes's formula

`Deﬁnition` The _conditional probability of F on E_ is denoted by

$Pr(F | E) = Pr(\text{F given that E has occurred})$

The probability that both E and F occur is related to the conditional probability of F on E by the formula

4.18

$ Pr(F | E) = \frac{Pr(F ∩ E)}{Pr(E)} $

`Proposition 4.24` _Let E and F be events_.

4.20

(a) $Pr(E) = Pr(E | F ) Pr(F ) + Pr(E | F^c ) Pr(F^c )$

4.21

(b) $Pr(E | F ) = \frac{Pr(F | E) Pr(E)}{Pr(F | E) Pr(E) + Pr(F | E^c ) Pr(E^c )}$, (Bayes's formula)

# 4.3.3 Monte Carlo algorithms

A Monte Carlo algorithm for property A takes as its input both a number m ∈ S to be tested and a randomly chosen number r and returns as output either Yes or No according to the following rules:

(1) If the algorithm returns Yes, then m deﬁnitely has property A. In conditional probability notation, this says that

$Pr(\text{m has property A }| \text{algorithm returns Yes}) = 1$

(2) If m has property A, then the algorithm returns Yes for at least 50% of the choices for r. Using conditional probability notation,

$Pr(\text{algorithm returns Yes} | \text{m has property A}) ≥ \frac{1}{2}$

# 4.3.4 Random Variables

`Deﬁnition` A random variable is a function

$X : Ω → R$

whose domain is the sample space Ω and that takes values in the real numbers.

`Deﬁnition` Let X : Ω → R be a random variable. The probability density function of X, denoted by f_{X} (x), is deﬁned to be

$f_X (x) = Pr(X = x)$

Remark 4.27\. In probability theory, people often use the distribution function of X, which is the function

$F_X (x) = Pr(X ≤ x)$,

`Deﬁnition` Let X and Y be two random variables. The joint density function of X and Y , denoted by $f_{X,Y} (x, y)$, is the probability that X takes the value x and Y takes the value y. Thus

$f_{X,Y} (x, y) = Pr(\text{X = x and Y = y})$

Similarly, the conditional density function, denoted by

$f_{X|Y} (x | y)$, is the probability that X takes the value x, given that Y takes the value y:

$f_{X|Y} (x | y) = Pr(X = x | Y = y).$

We say that X and Y are independent if

$f_{X,Y} (x, y) = f_X(x)f_Y(y)$ for all x and y.

`Theorem 4.33 (Bayes’s formula)`. Let X and Y be random variables and assume that $f_Y (y) > 0$. Then

$f_{X|Y} (x | y) = \frac{f_{X} (x)f_{Y |X} (y | x)}{f_Y (y)} $

_X and Y are independent_ ⇐⇒ $f_{X|Y} (x | y) = f_X (x)$ for all x and y$

`Deﬁnition`. A family of two or more random variables ${X1 , X2 , . . . , Xn }$ is independent if the events

$\set{X1 = x1\text{ and }X2 = x2\text{ and }· · ·\text{ and }Xn = xn } $

are independent for every choice of x1 , x2 , . . . , xn .

# 4.3.5 Expected value

Deﬁnition. Let X be a random variable that takes on the values $x1 , . . . , x_n$ . The _expected value (or mean) of_ X is the quantity

4.27

$E(X) = \sum_{i=1}^{n} xi · f_X (xi ) = \sum_{i=1}^{n} xi · Pr(X = xi ).$
