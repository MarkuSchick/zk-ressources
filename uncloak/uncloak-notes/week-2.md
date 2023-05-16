# Exercises

## Chapter 3 (p. 61)

### 1; How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?

With k=80 we have $2^{80}$ possible keys.The block cipher takes all possible k-bit inputs and maps them to 80 bit outputs. So we need $2^{80}$ 80-bit outputs. This is $2^{160}$ bits. So we need $2^{160}$ bits to store the table.
$
2^{160} bits = 2^{20} bytes = 2^{17} kilobytes = 2^{14} megabytes = 2^{11} gigabytes = 2^{8} terabytes =  256 terabytes
$

### 5; Suppose you have a processor that can perform a single DES encryption or decryption operation in seconds. Suppose you also have a large number of plaintext-ciphertext pairs for under a single unknown key. How many hours would it take, on average, to find that key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of processors?

dunno

### 6; Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. Give an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?

DES2(plain_text, key) = cipher


8; Familiarize yourself with a cryptographic CLI tools. A popular open source package is OpenSSL. Using an existing cryptographic library, decrypt the following ciphertext (in hex)

    53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07

with the following 256-bit key (also in hex):

    80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01

using AES.


9; Using an existing cryptography library, encrypt the following plaintext (in hex)

    29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D

with the following 256-bit key from problem 8, using AES. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

10; Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key and a plaintext and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.

Chapter 4 (p. 107)
1; Let be a plaintext and let be the length of in bytes. Let be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme:

Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number which satisfies and is a multiple of . Pad the plaintext by appending bytes, each with value .

3; Suppose you, as an attacker, observe the following 32-byte ciphertext (in hex)

46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D
2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75
and the following 32-byte ciphertext (also in hex)

51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30
7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B.
Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext corresponding to is

43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79
70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F.
What information, if any, can you infer about the plaintext corresponding
to ?

4; The ciphertext (in hex):

87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91
7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92
6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17
was generated with the 256-bit AES key (also in hex)

80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise.

6; Let , be a message that is two blocks long, and let
be a message that is one block long. Let be the encryption of using CBC mode with a random IV and a random key, and let
be the encryption of
using CBC mode with a random IV and the same key. Suppose an attacker knows and suppose the attacker intercepted and thus knows and
. Further suppose that, by random chance,
. Show that the attacker can compute
.

Implement a pair of functions: A PKCS 7 message padding function, and a padding validation function that takes a message and validates whether it has a correct padding.
