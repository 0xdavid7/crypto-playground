from py_ecc.bn128 import G1, multiply, add, eq, curve_order, field_modulus

print(G1)
# (1, 2)

print(add(G1, G1))
# (1368015179489954701390400359078579693043519447331113978918064868415326638035, 9918110051302171585080402603319702774565515993150576347155970296011118125764)

print(multiply(G1, 2))
#(1368015179489954701390400359078579693043519447331113978918064868415326638035, 9918110051302171585080402603319702774565515993150576347155970296011118125764)

# 10G + 11G = 21G
assert eq(add(multiply(G1, 10), multiply(G1, 11)), multiply(G1, 21))



#### Test failed ####

x = 5 # chosen randomly
# This passes

print("field_modulus: ", field_modulus)
print("curve_order: ", curve_order)
assert eq(multiply(G1, x), multiply(G1, x + curve_order))

## encoding rational

five_over_two = (5 * pow(2, -1, curve_order)) % curve_order
one_half = pow(2, -1, curve_order)

print("five_over_two: ", five_over_two)

# Essentially 5/2 = 2.5# 2.5 + 0.5 = 3
# but we are doing this in a finite field
assert eq(add(multiply(G1, five_over_two), multiply(G1, one_half)), multiply(G1, 3)) 