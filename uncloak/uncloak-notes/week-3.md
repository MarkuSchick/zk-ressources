course-2022-12-02 Session 3 Notes

# Ch 6:

Q3. Suppose a and b are both one block long, and suppose the sender MACs a, b, and a||b with CBC-MAC. An attacker who intercepts the MAC tags for these messages can now forge the MAC for the message
$ b || ( M(b) \oplus M(a) \oplus b) $
, which the sender never sent. The forged tag for this message is equal to M(a || b), the tag for a||b Justify mathematically why this is true.

## Background

We [review](theory/xor_cipher.md) the XOR operator $\oplus$ (also called "exclusive or" operator) to show this this is true.

To understand this review the definition of the CBC-MAC:

For messages P1, P2, ..., Pn, the CBC-MAC is defined as:

$$
\begin{aligned}

H_0 &:= IV \\
H_i &:= E_k(P_i \oplus H_{i-1}) \\
MAC &:= H_k

\end{aligned}
$$

, where IV = 0 and E_k is the encryption function under key k.

One particularity of that design is that for two messages a and b with M(a) = M(b) we also have M(a || c) = M(b || c). Here || denotes concatenate operator.

$M(a) = H_1 = E_k(a \oplus H_0) = E_k(a \oplus IV) = E_k(a \oplus 0) = E_k(a)$

For a concatenation of a and c, we have:

$M(a || c) = H_2 = E_k(c \oplus H_1) = E_k(c \oplus M(a))$

it is easy to see that $M(a || c) = M(b || c)$

$M(b || c) = H_2 = E_k(c \oplus H_1) = E_k(c \oplus M(b))$

## Solution

We know MAC(a), MAC(b) and MAC(a || b)

$$
\begin{aligned}

t_a      &= E_k(a) \\
t_b      &= E_k(b) \\
t_{a||b} &= E_k(b \oplus t_a) \\
\end{aligned}
$$

Show that $ M(b || ( M(b) \oplus M(a) \oplus b) ) $ = M(a || b)

From the definition of CBC-MAC:

$$
\begin{aligned}


MAC(m) &= M(b || ( M(b) \oplus M(a) \oplus b) ) \\
       \text{since $M(a∣∣c) = E_k(c \oplus M(a)$ } \\
       &= E_k(M(b) \oplus M(a) \oplus b \oplus M(a))  \\
       \text{replace $M(i)$ with $t_i$} \\
       &= E_k(t_b \oplus t_a\oplus b\oplus t_b) \\
       \text{use $(A \oplus B) \oplus C = A \oplus (B \oplus C)$ } \\
       &= E_k( t_a\oplus b) \\
       \text{by definition} \\
       &= t_{a||b} \\


\end{aligned}
$$

`Suppose message a is one block long. Suppose that an attacker has received the MAC t for a using CBC-MAC under some random key unknown to the attacker. Explain how to forge the MAC for a two-block message of your choice. What is the two-block message that you chose? What is the tag that you chose? Why is your chosen tag a valid tag for your two-block message?`

You could use m = 0.
Since $t = E_k(a)$

$$
\begin{aligned}

MAC &:= E_k(m \oplus t) = E_k(0 \oplus t) \\

\end{aligned}
$$
