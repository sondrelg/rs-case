[![pypi](https://img.shields.io/pypi/v/rscase.svg)](https://pypi.org/project/rscase/)
[![tests](https://github.com/sondrelg/rs-case/workflows/tests/badge.svg)](https://github.com/sondrelg)
[![codecov](https://codecov.io/gh/sondrelg/rscase/branch/master/graph/badge.svg)](https://codecov.io/gh/sondrelg/rscase)

![Rust](https://img.shields.io/badge/Rust-v1.41.0-orange.svg)
![Py](https://img.shields.io/badge/Python-v3.8-blue.svg)


# rscase

This module is a simple python package implemented in [Rust](https://www.rust-lang.org/learn), using [pyo3](https://github.com/PyO3/pyo3) to access binding for the python interpreter. This was primarily built for fun, and the actual usefulness of the implementation has not been a primary focus.

In terms of function, the package provides utility functions for generating strings formatted in several different case standards. The standards available are listed below.

| Case name        | Format example           |
| :--------------: |:-----------------:|
| camel case       | camelCasedValue   |
| snake case       | snake_cased_value |
| pascal case      | PascalCasedValue  |
| kebab case       | kebab-cased-value |
| train case       | TRAIN-CASED-VALUE |

While it is very common for systems to convert, e.g., `snake_case` to `camelCase`, it doesn't really make sense to convert `TRAIN-CASE` to itself, and it's not something you should look to use this package for. If you use the module for *anything*, you should keep in mind that the logic is built around converting string from snake/camel case to another format. Converting `"TRAIN-CASE"` to train case will therefore yield `"T-R-A-I-N--C-A-S-E"`. 

## Install 

```shell
pip install rscase
```

## Usage

```python
from rscase import camel_case

camel_case('this_is-a_Test')
>> thisIsATest
```


## Benchmarking Performance

Because the functions contained in this package only do some very simple string manipulation, they seem like they might actually be good candidates for a performance test - so why not do one.
 
To make a test fair, and to make sure we're comparing apples to apples, I decided to test the Rust function `snake_case` to its Python twin, shown below.

```python
from rscase import rscase

test_string = "thisIsALongCamelCasedAlphabeticKey"

# Test functions 

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
```

The main difference between the two functions is that Rust requires you to create a vector of `char`'s to iterate through a string, while in Python you don't.

### Results

After running the tests, the results seems to be pretty promising in favor of the rust implementation. 

| Reps | Rust Execution Time | Python Execution Time | Difference |
| :--------------: |:-----------------:| :--------------: |:-----------------:|
| 1 | 18.30 μs | 14.20 μs | 0.78x |
| 10 | 55.20 μs | 114.20 μs | 2.07x |
| 100 | .49 ms | 1.11 ms | 2.27x |
| 1000 | 4.88 ms | 11.18 ms | 2.28x |
| 10 000 | 47.20 ms | 109.13 ms | 2.31x |
| 100 000 | .47 s | 1.08 s | 2.31x |
| 1000 000 | 4.83 s | 11.12 s | 2.30x |
| 10 000 000 | 46.67 | 109.27 s | 2.34x |
| 100 000 000 | 484 s | 1102 s | 2.28x |

After only 100 reps, the results seem to stabilize, and flatten out at around a 2.3x longer execution time for the Python implementation.

The 1-rep result however, seems to show that Python performs better in the scenario that would normally matter. For anyone worried about this, I think there's no doubt that one test of 1 rep is a poor experiment, and so I ran this scenario another one million times. With a larger sample, the average improvement for `1 rep` averages to `1.85x` while the median becomes `1.88x`. In short, the Rust implementation seems to outperform the Python across the board.


## Contributing

Contributions are welcome. 

Look in the docs folder for more info.