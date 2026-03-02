def vector_addition(v1, v2):
    return [a + b for a, b in zip(v1, v2)]

def scalar_multiplication(v, scalar):
    return [a*scalar for a in v]