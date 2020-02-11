#######
rs-case
#######

.. image:: https://img.shields.io/pypi/v/rscase.svg
    :target: https://pypi.org/project/rscase/

.. image:: https://github.com/sondrelg/rs-case/workflows/tests/badge.svg
    :target: https://github.com/sondrelg/rs-case

.. image:: https://img.shields.io/badge/Python-v3.8-blue.svg
    :target: https://github.com/sondrelg/rs-case

.. image:: https://img.shields.io/badge/Rust-v1.43.0--nightly-red.svg
    :target: https://github.com/sondrelg/rs-case

.. image:: https://codecov.io/gh/sondrelg/rscase/branch/master/graph/badge.svg
    :target: https://github.com/sondrelg/rs-case

This module offers a handful of case-formatting utility functions. It is a very simple Python package, written in Rust_ and implemented using pyo3_ which offers you easy Rust bindings for the Python interpreter.

.. _Rust: https://www.rust-lang.org/learn
.. _pyo3: https://github.com/PyO3/pyo3

Installation
############

Install with pip using:

.. code:: shell

    pip install rscase

Usage
#####

The package provides utility functions for generating strings formatted in several different case standards. 

The case-standards and their functions are listed below.

+-----------------+-------------+-------------------+
| Supported cases | Function    | Format example    |
+=================+=============+===================+
|    camel case   | camel_case  | camelCasedValue   |
+-----------------+-------------+-------------------+
|    snake case   | snake_case  | snake_cased_value |
+-----------------+-------------+-------------------+
|    pascal case  | pascal_case | PascalCasedValue  |
+-----------------+-------------+-------------------+
|    kebab case   | kebab_case  | kebab-cased-value |
+-----------------+-------------+-------------------+
|    train case   | train_case  | TRAIN-CASED-VALUE |
+-----------------+-------------+-------------------+

All functions are imported and accessed the same way:

.. code:: shell

    >> [in] from rscase import rscase
    >> [in] rscase.camel_case('this_is-a_Test')
    >> [out] thisIsATest


If you want to use this package, please note that the case functions are written to successfully convert camel case and snake case to the remaining formats. Formatting train case to itself doesn't really make sense, and the way I would use this would be to, e.g., serialize out response data to a camelCased format.

Benchmarking Performance
########################

This repo is a bit of an experiment, and because the functions contained in this package only do some very simple string manipulation, they seem like they might actually be good candidates for Python vs Rust performance benchmarking.
 
To try and make this a fair comparison - to make sure we're comparing apples to apples - I decided to test the Rust function `snake_case` (see the Rust function [here](src/lib.rs)) to an identical Python function. The Python version is shown below:

.. code:: python

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


The main difference between the two functions, flow-wise, is only that Rust won't let you just iterate over a string, so you have to create a vector of `char`'s instead - or at least that's how I did it.

### Results

After running the tests, the results seems to be pretty promising - in favor of the Rust implementation. 

+-------------+---------------------+-----------------------+------------+
| Reps        | Rust Execution Time | Python Execution Time | Difference |
+=============+=====================+=======================+============+
| 1           | 18.30 μs            | 14.20 μs              | 0.78x*     |
+-------------+---------------------+-----------------------+------------+
| 10          | 55.20 μs            | 114.20 μs             | 2.07x      |
+-------------+---------------------+-----------------------+------------+
| 100         | .49 ms              | 1.11 ms               | 2.27x      |
+-------------+---------------------+-----------------------+------------+
| 1000        | 4.88 ms             | 11.18 ms              | 2.28x      |
+-------------+---------------------+-----------------------+------------+
| 10 000      | 47.20 ms            | 109.13 ms             | 2.31x      |
+-------------+---------------------+-----------------------+------------+
| 100 000     | .47 s               | 1.08 s                | 2.31x      |
+-------------+---------------------+-----------------------+------------+
| 1000 000    | 4.83 s              | 11.12 s               | 2.30x      |
+-------------+---------------------+-----------------------+------------+
| 10 000 000  | 46.67 s             | 109.27 s              | 2.34x      |
+-------------+---------------------+-----------------------+------------+
| 100 000 000 | 484 s               | 1102 s                | 2.28x      |
+-------------+---------------------+-----------------------+------------+

The results are pretty clear: after only 100 reps, the results seem to stabilize, and flatten out at around a 2.3x longer execution time for the Python implementation.

*the 1-rep result seems to show that Python actually outperforms Rust in the scenario that would normally *actually* matter. Since it makes sense that variance would be high when trying to measure something at the microsecond level I decided to run this individual scenario again, another one million times, to increase the sample size. With a larger sample, the average `difference` for 1 rep averages to `1.85x` slower in Python, and the median is `1.88x`. In short, the Rust implementation seems to outperform the Python across the board.
