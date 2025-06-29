import numpy as np
import galois
from functools import reduce

# out = x⁴ - 5y²x²


# This will break down as

# v1 = x * x
# v2 = v1 * v1         # x^4
# v3 = -5y * y
# -v2 + out = v3*v1    # -5y^2 * x^2


# 1, out, x, y, v1, v2, v3
L = np.array([
    [0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, -5, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1],
])

R = np.array([
    [0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
])

O = np.array([
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, -1, 0],
])

print(f"L:\n{L}\n")
print(f"R:\n{R}\n")
print(f"O:\n{O}\n")

x = 4
y = -2
v1 = x * x
v2 = v1 * v1         # x^4
v3 = -5*y * y
out = v3*v1 + v2    # -5y^2 * x^2

witness = np.array([1, out, x, y, v1, v2, v3])

print(f"witness:\n{witness}\n")

Ls = np.matmul(L, witness)
Rs = np.matmul(R, witness)
Os = np.matmul(O, witness)


print(f"Ls:\n{Ls}\n")
print(f"Rs:\n{Rs}\n")
print(f"Os:\n{Os}\n")

assert all(np.equal(Ls * Rs, Os)), "Does not match"

## Finite field arithmetic

GF = galois.GF(79)

a = GF(70)
b = GF(10)

assert a + b == GF(1), "Does not match"


# Because in FF -5 -> 79 - 5 = 74; -1 -> 79 - 1 = 78

L = np.array([
    [0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 74, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1],
])

R = np.array([
    [0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
])

O = np.array([
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 78, 0],
])

L_galois = GF(L)
R_galois = GF(R)
O_galois = GF(O)

print(f"L_galois:\n{L_galois}\n")
print(f"R_galois:\n{R_galois}\n")
print(f"O_galois:\n{O_galois}\n")


x = GF(4)
y = GF(79-2) # we are using 79 as the field size, so 79 - 2 is -2
v1 = x * x
v2 = v1 * v1         # x^4
v3 = GF(79-5)*y * y
out = v3*v1 + v2    # -5y^2 * x^2

print(f"x: {x}, y: {y}, v1: {v1}, v2: {v2}, v3: {v3}, out: {out}")

witness = GF(np.array([1, out, x, y, v1, v2, v3]))

Ls = np.matmul(L_galois, witness)
Rs = np.matmul(R_galois, witness)
Os = np.matmul(O_galois, witness)

assert all(np.equal(Ls * Rs, Os)), "Does not match"
print("All good")


# Polynomial interpolation in finite fields

def interpolate_column(col):
    xs = GF(np.array([1,2,3,4]))
    return galois.lagrange_poly(xs, col)

# axis 0 is the columns. apply_along_axis is the same as doing a for loop over the columns and collecting the results in an array
U_polys = np.apply_along_axis(interpolate_column, 0, L_galois)
V_polys = np.apply_along_axis(interpolate_column, 0, R_galois)
W_polys = np.apply_along_axis(interpolate_column, 0, O_galois)

## -> we have 4 constraints, so we have the degree of the polynomial as 3, and we have 7 coefficients
## so we have 4 polynomials of degree 3 with 7 coefficients
print("\nU_polys: ")
for i in range(len(U_polys)):
    print(f"U[{i}]= {U_polys[i]}")
    
print("\nV_polys:")
for i in range(len(V_polys)):
    print(f"V[{i}]= {V_polys[i]}")

print("\nW_polys:")
for i in range(len(W_polys)):
    print(f"W[{i}]= {W_polys[i]}")
    
# Computing h(x)
# We already know t(x) will be (x - 1)(x - 2)(x - 3)(x - 4) since there are four rows.

# By way of reminder, this is the formula for a Quadratic Arithmetic Program. The variable a is the witness (1, a₁, …, aₘ).

def inner_product_polynomials_with_witness(polys, witness):
    mul_ = lambda x, y: x * y
    sum_ = lambda x, y: x + y
    return reduce(sum_, map(mul_, polys, witness))

 # term_1 = U.a, term_2 = V.a, term_3 = W.a
term_1 = inner_product_polynomials_with_witness(U_polys, witness)
term_2 = inner_product_polynomials_with_witness(V_polys, witness)
term_3 = inner_product_polynomials_with_witness(W_polys, witness)

print(f"term_1: {term_1}")
print(f"term_2: {term_2}")
print(f"term_3: {term_3}")

# t = (x - 1)(x - 2)(x - 3)(x - 4)
t = galois.Poly([1, 78], field = GF) * galois.Poly([1, 77], field = GF) * galois.Poly([1, 76], field = GF) * galois.Poly([1, 75], field = GF)

h = (term_1 * term_2 - term_3) // t

assert term_1 * term_2 == term_3 + h * t, "division has a remainder"

print(f"t: {t}")
print(f"h: {h}")