from vector import vector_addition, scalar_multiplication, dot_product

def test_vector_addition():
    v1 = [1, 2]
    v2 = [4, 5]

    assert vector_addition(v1, v2) == [5, 7]
    print("Test passed")

    v3 = [1,2,3]
    v4 = [1,2,3]

    assert vector_addition(v3, v4) == [2, 4, 6]
    print("Test passed")

def test_scalar_multiplication():
    v = [5, 3]
    scalar = 4

    assert scalar_multiplication(v, scalar) == [20, 12]
    print("Test passed")

def test_dot_product():
    v1 = [1, 2]
    v2 = [4, 5]

    assert dot_product(v1, v2) == 14
    print("Test passed")