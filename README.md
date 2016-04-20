# Share a Secret

** ⚠️ !WARNING! ⚠️ ** 

As this code has not been audited by a third-party, I would not advise use of this tool for any sensitive secrets. Also, since this is early version `alpha` software, the encoding may be subject to change, and secrets shared now, may not be decodable with future versions of this software 

** ⚠️ !WARNING! ⚠️ **

[![Linux Build Status](https://travis-ci.org/KyleChamberlin/share_a_secret.svg?branch=master)](https://travis-ci.org/KyleChamberlin/share_a_secret) 
[![Windows Build status](https://ci.appveyor.com/api/projects/status/twygyh6hhjss2deb/branch/master?svg=true)](https://ci.appveyor.com/project/KyleChamberlin/share-a-secret/branch/master)
[![Coverage Status](https://coveralls.io/repos/github/KyleChamberlin/share_a_secret/badge.svg?branch=master)](https://coveralls.io/github/KyleChamberlin/share_a_secret?branch=master)
[![RTFD](https://readthedocs.org/projects/mail-manager-python-interface/badge/?version=latest)](http://share_a_secret.rtfd.org/en/latest)
[![CLA assistant](https://cla-assistant.io/readme/badge/KyleChamberlin/share_a_secret)](https://cla-assistant.io/KyleChamberlin/share_a_secret)

Share a Secret is a [Rust][] implementation of a threshold [Shamir's secret sharing scheme][] which aims to produce pronouncable shares.

## Goals

The main goal for this application is to split a secret of an arbitrary length in `n` different shares where at least `k` (where `k <= n`) shares are required to recover it. 
We assume that our adversary will only be able to compromise at most `k-1` shares. Shares should be kept offline. 

A typical use case would be splitting an AES-256 key to a sensitive volume.

## Why Shamir?

The Shamir's Secret Sharing scheme has been chosen for this implementation for the following reasons.

### [Information-theoretic security][]

Shamir's secret sharing is known to have the perfect secrecy property.
In the context of `(k,n)`-threshold schemes this means that if you have
less than `k` shares available, you have absolutely no information about
what the secret is except for its length (typical secrets would be an AES-256 key, all have the same length).

Information-theoretic security gives us strong guarantees:

- 1) That there are provably no faster attacks than brute force exhaustion of key space.
- 2) An encryption protocol that has information-theoretic security does not depend for its effectiveness on unproven assumptions about computational hardness, and such an algorithm is not vulnerable to future developments in computer power such as quantum computing. 


### Peer-review

The Shamir secret sharing scheme has been around since 1979 and has been [well studied][Google Scholar].

## Usage

### Structure of a share

```
  2-1-placeholdertex
  ^ ^ ^^^^^^^^^^^^^^
  K N        D        
```

A share is built out of three parts separated with a dash: K-N-D.

- K specifies the number of shares necessary to recover the secret.
- N is the identifier of the share and varies between 1 and n where n is the total number of generated shares.
- The D part is a Base64 encoding of a specific share's raw data.

### Command-line encoding

Passing a secret to rustysecrets for encoding:

```
$ echo a secret to share | ./share_a_secret -k2 -n5
2-1-placeholdertex
2-2-placeholdertex
2-3-placeholdertex
2-4-placeholdertex
2-5-placeholdertex
```

Two parameters will be required, `-k` for the minimum number of shares required to recover the secret, and `-n` for the total number of shares to generate.

Decoding a subset of shares (one share per line) can be done like this:

```
$ echo -e "2-2-placeholdertex \n 2-4-placeholdertex" | ./share_a_secret -d
a secret to share
```

[Rust]:                           https://www.rust-lang.org 
                                  "Rust Language Website"

[Shamir's secret sharing scheme]: https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing 
                                  "Wikipedia"

[Information-theoretic security]: https://en.wikipedia.org/wiki/Information-theoretic_security 
                                  "Wikipedia"

[Google Scholar]:                 https://scholar.google.ch/scholar?cites=12714240754634232446&as_sdt=2005&sciodt=0,5&hl=en 
                                  "Scholarly papers on Shamir's Secret sharing - Google Scholar"
