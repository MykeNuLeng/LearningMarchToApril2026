def vector_addition(v1, v2):
    return [a + b for a, b in zip(v1, v2)]

def scalar_multiplication(v, scalar):
    return [a*scalar for a in v]

def dot_product(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))

def matrix_multiplication(m1, m2):
    # [[a,c],[b,d]] * [[e,g],[f,h]] = [[a*e + b*g, c*e + d*f], [a*f + b*h, c*f + d*h]]
    # = [[r1m1.c1m2, r2m1.c1m2], [r1m1.c2m2, r2m1.c2m2]]
    r1m1 = [m1[0][0], m1[1][0]]
    r2m1 = [m1[0][1], m1[1][1]]
    c1m2 = m2[0]
    c2m2 = m2[1]

    return [[dot_product(r1m1, c1m2), dot_product(r2m1, c1m2)], [dot_product(r1m1, c2m2), dot_product(r2m1, c2m2)]]