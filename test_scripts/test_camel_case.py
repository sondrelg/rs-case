import timeit

from rscase import rscase

test_string = "this_is_a_long_camel_cased_alphabetic_key"


def original_camel_case():
    string = test_string
    new_string = ""
    upper_next = False
    for index in range(len(string)):
        if index == 0:
            new_string += string[index].lower()
        elif upper_next and string[index].isalpha():
            new_string += string[index].upper()
            upper_next = False
        elif string[index] == "-" or string[index] == '_':
            upper_next = True
        else:
            new_string += string[index]
    return new_string


def rust_camel_case():
    string = test_string
    return rscase.camel_case(string)


for iterations in [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000]:
    python = timeit.timeit(original_camel_case, number=iterations)
    rust = timeit.timeit(rust_camel_case, number=iterations)
    print(f"Python runtime for {iterations} reps: {python}")
    print(f"Rust   runtime for {iterations} reps: {rust}\n")
    print(f'Rust implementation is {python / rust}x faster\n\n')
