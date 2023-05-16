# XOR cipher

In cryptography, we make heavy use of the XOR also called "exclusive or" operator. It is a binary operator that takes two bits and returns 1 iff either of the bits is 1 but not both. It is denoted by $\oplus$.

That is very helpful as it allows us to encrypt and decrypt a message by XORing it with a key. This is called a XOR cipher.

## XOR cipher example

Encryption:

$$
\begin{aligned}
m := 10111 \\
k := 01100 \\

c
:= 11011   \\
= m \oplus k \\

\end{aligned}
$$

Decryption:

$$
\begin{aligned}
c := 11011 \\
k := 01100 \\

m
:= 10111 \\
:= c \oplus k \\

qed

\end{aligned}
$$

see: [notation](notation.md) for the notation

## Useful math rules

$$
\begin{aligned}

A \oplus 0 &= A \\
A \oplus A &= 0, \\
A \oplus B &= B \oplus A, \\
(A \oplus B) \oplus C &= A \oplus (B \oplus C), \\
(B \oplus A) \oplus A &= B \oplus 0 = B, \\

\end{aligned}
$$
