# Exercises

## Ch 1:

`Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.`

One example is using a timelock in a smart contract. On the one hand the time-lock can stop a hacker trying to drain the contract. On the other hand leaking
the private key of the privileged account can allow a hacker to DDOS the contract or potentially change other market metrics (such as funding rates).

## Ch 2:

`Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.`

In a symmetric-key system each person needs to exchange a unique key with anyone else. Each person has to exchange 29 keys. So 29 \* 30 / 2 = 435 keys need to be exchanged.

reference: [Triangular Numbers](theory/triangular_numbers.md)

`Q4. Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.`
We must assume that only Alice has access to their private key and neither the signing algorithm nor that particular signature have been compromised. If this is the case then yes, it proves that Alice saw the message and chose to sign it.

`Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?`
No even leaking only part of the identifying can be dangerous. Information can then be used decrypt at least part of the messages what could become useful.

`Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.`

We will often talk about this as the $2^{n/2}$ bound, or the birthday bound. For $n^128$ bits of security we need $2^{n/2}$ keys. So $n = 256$.

## General:

`Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help: https://cryptography.rs/ https://lib.rs/ (librs is equivalent to crates.io, with a different interface)`

`Benchmark the speed of an algorithm in the two different implementations with Criterion.`

`You’re implementing a Tweakable Encryption scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.`

`You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?`

`Implement the Vignère cipher in 100 lines or less.`

`What is a side channel attack? Is your cipher implementation constant time?`

`Extra: Read New Directions in Cryptography.`

`Extra: Consider ways to contribute what you learned this week to the Uncloak knowledge graph.`
