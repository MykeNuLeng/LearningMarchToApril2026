from test_vector import test_vector_addition, test_scalar_multiplication, test_dot_product, test_matrix_multiplication

if __name__ == "__main__":
    print("Running vector tests")
    test_vector_addition()
    print("Running scalar tests")
    test_scalar_multiplication()
    print("running dot product tests")
    test_dot_product()
    print("Running matrix multiplication tests")
    test_matrix_multiplication()