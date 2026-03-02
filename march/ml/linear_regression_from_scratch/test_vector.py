from vector import vector_addition, scalar_multiplication

def test_vector_addition():
    v1 = [1, 2]
    v2 = [4, 5]

    assert vector_addition(v1, v2) == [5, 7]

    v3 = [1,2,3]
    v4 = [1,2,3]

    assert vector_addition(v3, v4) == [2, 4, 6]

def test_scalar_multiplication():
    v = [5, 3]
    scalar = 4

    assert scalar_multiplication(v, scalar) == [20, 12]

test_vector_addition()
test_scalar_multiplication()
