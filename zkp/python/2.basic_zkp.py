# Basic zero knowledge proofs with elliptic curves
# Consider this rather trivial example:


# Claim: “I know two values x and y such that x + y = 15”


# Proof: I multiply x by G1 and y by G1 and give those to you as A and B.

# Verifier: You multiply 15 by G1 and check that A + B == 15G1.

# Here it is in python

from py_ecc.bn128 import G1, multiply, add, curve_order

# Prover
secret_x = 5
secret_y = 10

x = multiply(G1, 5)
y = multiply(G1, 10)

proof = (x, y, 15)

print("proof: ", proof)

# verifier
if multiply(G1, proof[2]) == add(proof[0], proof[1]):
    print("statement is true")
else:
    print("statement is false")
    
    
### 23x = 161
# Prover

secret_x = 7

x = multiply(G1, 7)

proof = (x, 23, 161)

print("proof: ", proof)

# verifier
if multiply(G1, proof[2]) == multiply(proof[0], proof[1]):
    print("statement is true")
else:
    print("statement is false")
    