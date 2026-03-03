def vector_addition(v1, v2):
    return [a + b for a, b in zip(v1, v2)]

def scalar_multiplication(v, scalar):
    return [a*scalar for a in v]

def dot_product(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))

def matrix_multiplication(m1, m2):
     #[[a,c],[b,d]] * [[e,g],[f,h]] = [[a*e + b*g, c*e + d*f], [a*f + b*h, c*f + d*h]]
    return m1