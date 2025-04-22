from py_ecc.bn128 import G1, multiply, add,neg, curve_order, eq, Z1
from functools import reduce
import galois


# Prover
x = 5

X3 = multiply(G1, 5**3)
X2 = multiply(G1, 5**2)
X = multiply(G1, 5)

# Verifier
left_hand_side = multiply(G1, 39)
right_hand_side = add(add(add(multiply(X3, 1),
                              multiply(neg(X2), 4)),
                              multiply(X, 3)),
                              multiply(neg(G1), 1))

assert eq(left_hand_side, right_hand_side), "lhs ≠ rhs"



################ Trusted setup #####################

print("initializing a large field, this may take a while...")
GF = galois.GF(curve_order)
print("GF: ", GF)

def inner_product(ec_points, coeffs):
    return reduce(add, (multiply(point, int(coeff)) for point, coeff in zip(ec_points, coeffs)), Z1)

def generate_powers_of_tau(tau, degree):
    result = []
    print("tau: ", tau)
    print("degree: ", degree)
    for i in range(degree + 1):
        # x = G1 * tau^i
        print("i: ", i)
        x = multiply(G1, int(tau ** i))
        print("x: ", x)
        result.append(x)
        print("=====\n")
    return result

# p = (x - 4) * (x + 2)
p = galois.Poly([1, -4], field=GF) * galois.Poly([1, 2], field=GF)
print("p: ", p)

# evaluate at 8
tau = GF(8)
print("tau: ", tau)

# evaluate then convert
powers_of_tau = generate_powers_of_tau(tau, p.degree)
print(f"powers_of_tau: {powers_of_tau}\n")

evaluate_then_convert_to_ec = multiply(G1, int(p(tau)))

# evaluate via encrypted evaluation# coefficients need to be reversed to match the powers
evaluate_on_ec = inner_product(powers_of_tau, p.coeffs[::-1])

if eq(evaluate_then_convert_to_ec, evaluate_on_ec):
    print("elliptic curve points are equal")

    
#####################################
# p = x^2 - 2x - 8
# -2 = 21888242871839275222246405745257275088548364400416034343698204186575808495615
# -8 = 21888242871839275222246405745257275088548364400416034343698204186575808495609 


# initializing a large field, this may take a while...
# GF:  <class 'galois.GF(21888242871839275222246405745257275088548364400416034343698204186575808495617)'>
# p:  x^2 + 21888242871839275222246405745257275088548364400416034343698204186575808495615x + 21888242871839275222246405745257275088548364400416034343698204186575808495609
# tau:  8
# elliptic curve points are equal