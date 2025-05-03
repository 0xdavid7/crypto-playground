// The R1CS is a system of equations that looks like this:
// c = a * b + 1
// The goal is to find the matrices O, L, R such that:
// O = L * R
// Example: r = x * y * z * u
// Convert to:
// v1 = xy
// v2 = zu
// r = v1 * v2
// Witness vector: [1, r, x, y, z, u, v1, v2]
// L = [0, 0, 1, 0, 0, 0, 0, 0
//      0, 0, 0, 0, 1, 0, 0, 0
//      0, 0, 0, 0, 0, 0, 1, 0 ]
// R = [0, 0, 0, 1, 0, 0, 0, 0
//      0, 0, 0, 0, 0, 1, 0, 0
//      0, 0, 0, 0, 0, 0, 0, 1 ]
// O = [0, 0, 0, 0, 0, 0, 1, 0
//      0, 0, 0, 0, 0, 0, 0, 1
//      0, 1, 0, 0, 0, 0, 0, 0 ] 