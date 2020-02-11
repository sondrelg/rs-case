import timeit

test_string = "thisIsALongCamelCasedAlphabeticKey"


def original_snake_case():
    string = test_string
    new_string = ""
    dash = "-"
    for index in range(len(string)):
        if index == 0:
            new_string += string[index].lower()
        elif string[index] == dash:
            new_string += '_'
        elif string[index].upper() == string[index]:
            new_string += f'_{string[index]}'
        else:
            new_string += string[index]
    return new_string


def rust_snake_case():
    string = test_string
    return rscase.snake_case(string)


for iterations in [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000]:
    python = timeit.timeit(original_snake_case, number=iterations)
    rust = timeit.timeit(rust_snake_case, number=iterations)
    print(f"Python runtime for {iterations} reps: {python}")
    print(f"Rust   runtime for {iterations} reps: {rust}\n")
    print(f'Rust implementation is {python / rust}x faster\n\n')
