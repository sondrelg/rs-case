[![pypi](https://img.shields.io/pypi/v/rscase.svg)](https://pypi.org/project/rscase/)
[![tests](https://github.com/sondrelg/rs-case/workflows/tests/badge.svg)](https://github.com/sondrelg)
[![codecov](https://codecov.io/gh/sondrelg/rscase/branch/master/graph/badge.svg)](https://codecov.io/gh/sondrelg/rscase)

![Rust](https://img.shields.io/badge/Rust-v1.41.0-orange.svg)
![Py](https://img.shields.io/badge/Python-v3.8-blue.svg)


# rscase - a simple collection of string case manipulation helpers

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

## Benchmark

The most common reason someone would normally look to implement python packages in different languages has to be the performance aspect. Since this implementation only contains some very simple string manipulation, it seems like it might *actually* be a useful thing to use for benchmarking.
 
Out of the few functions contained in the `src/lib.rs` I chose to test the `snake_case` function, and used the python `timeit` library to run and time simulations. 

To make sure we're comparing apples to apples, I write same function in python, using more or less identical logic.

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

The main difference between the two functions is that Rust requires you to create a vector of `char`'s to iterate through i string. You would never do this in Python, and I've left that part of it out of the Python implementation.

### Results


| Reps | Rust Execution Time | Python Execution Time | Difference |
| :--------------: |:-----------------:| :--------------: |:-----------------:|
| 1 | 18.30 μs | 14.20 μs | <a style="color:green">0.78x</a> |
| 10 | 55.20 μs | 114.20 μs | <a style="color:red">2.07x</a> |
| 100 | .49 ms | 1.11 ms | <a style="color:green">2.27x</a> |
| 1000 | 4.88 ms | 11.18 ms | <a style="color:green">2.28x</a> |
| 10 000 | 47.20 ms | 109.13 ms | <a style="color:green">2.31x</a> |
| 100 000 | .47 s | 1.08 s | <a style="color:green">2.31x</a> |
| 1000 000 | 4.83 s | 11.12 s | <a style="color:green">2.30x</a> |
| 10 000 000 | 46.67 | 109.27 s | <a style="color:green">2.34x</a> |
| 100 000 000 | 484 s | 1102 s | <a style="color:green">2.28x</a> |

Results were pretty promising in favor of the rust implementation, with a performance improvement (execution time) of ~2.25 once reps reach a certain volume. 

The `1 rep` result *seems* poor at first, but doesn't mean anything in isolation, because there will be some variance in results - especially when you're talking about microseconds. After running the `1 rep` scenario on repeat another one million times, the average and median improvements for a `1 rep` simulation changes to `1.85x` and `1.88x` respectively. All in all, pretty decent.



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


## Contributing

Contributions are welcome. 

Look in the docs folder for more info.