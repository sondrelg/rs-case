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

Feel free to submit a PR if you have suggestions for altered behavior that might benefit you.