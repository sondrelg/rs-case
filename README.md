[![pypi](https://img.shields.io/pypi/v/rscase.svg)](https://pypi.org/project/rscase/)
[![Pipeline](https://github.com/sondrelg/rscase/workflows/Pipeline/badge.svg)](https://github.com/sondrelg)
[![codecov](https://codecov.io/gh/sondrelg/rscase/branch/master/graph/badge.svg)](https://codecov.io/gh/sondrelg/rscase)

![Py](https://img.shields.io/badge/Python-v3.8-blue.svg)
![Rust](https://img.shields.io/badge/Rust-v1.41.0-orange.svg)


# rscase - a small collection of string case manipulation helpers

rscase is a simple python package implemented in [Rust](https://www.rust-lang.org/learn), using [pyo3](https://github.com/PyO3/pyo3) to access Rust binding for the python interpreter. 

Package provides utility functions for generating the following cases:

- camelCase
- snake_case
- PascalCase
- kebab-case
- TRAIN-CASE

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
