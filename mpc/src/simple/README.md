# MPC BLS Signature Implementation
This wallet implements a **simple** threshold BLS signature scheme using MPC. 

## 📛 This is just for educational purpose and not suitable for production. 
## 📛 I need more time to understand and deep dive into this, specially on the verification part.

## Key parts:
- **Key Gen**: Gen key shares based on Shamir's Secret Sharing
- **Signing**: Create signature shares for each party
- **Signature Aggregation**: Combine sig shares  
- **Signature Verification**: Verifies the aggregated signature against the group's public key


### Notes:

The key generation follows: 
1. Generates a random polynomial of degree (threshold - 1):
   ```
   f(x) = share_0 + share_1 * x + share_2 * x² + ... + share_{t-1} * x^{t-1}
   ```
   where share_0 is the secret key

2. Each participant i receives:
   - A secret share f(i)
   - A public key share g^{f(i)}

3. The group's public key is g^{share_0}

### Signature Creation

1. Each participant i creates a signature share:
   ```
   sig_i = H(msg) * secret_share_i, where H(msg) and share_i are on G1
   ```

2. Signature shares are combined using Lagrange interpolation:
   ```
   signature = ∑(sig_i * l_i(0))
   ```

### Verification

The signature is verified using bilinear pairings:
```
e(signature, g) = e(H(message), public_key),
```
whrere e is a bilinear pairing, signature is on G1, 
g is on G2, and H(message) is on G1, public_key is on G2.

## Usage Example

```rust
// Initialize a new MPC wallet
let threshold = 3;
let total_participants = 5;
let (wallet, shares) = MPCWallet::keygen(threshold, total_participants, &mut rng)?

// Sign a message (requires threshold participants)
let message = b"Hello, MPC!";
let sig_share1 = MPCWallet::sign_share(message, &shares[0]);
let sig_share2 = MPCWallet::sign_share(message, &shares[1]);
let sig_share3 = MPCWallet::sign_share(message, &shares[2]);

// Combine signature shares
let signature = wallet.combine_signature_shares(&[sig_share1, sig_share2, sig_share3])?;

// Verify the signature
MPCWallet::verify(&wallet.public_key, message, &signature)?
```

## Dependencies

- `ark-bls12-381`: BLS12-381 curve implementation
- `ark-ec`: Elliptic curve operations
- `ark-ff`: Finite field arithmetic
- `sha2`: Secure hash function

