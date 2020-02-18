#!/usr/bin/env python
import sys

from setuptools import setup, find_packages

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

from rscase import __version__, __author__

with open('README.rst') as readme_file:
    readme = readme_file.read()

setup_requires = ["setuptools-rust>=0.10.1", "wheel", "setuptools"]

setup(
    name="rscase",
    version=__version__,
    description="Python package for string case formatting; implemented in Rust.",
    py_modules=['rscase'],
    include_package_data=True,
    long_description=readme,
    long_description_content_type='text/x-rst',
    license="BSD",
    author=__author__,
    author_email="sondrelg@live.no",
    url="https://github.com/sondrelg/rs-case",
    download_url='https://pypi.python.org/pypi/rscase',
    packages=find_packages(exclude=['']),
    install_requires=setup_requires,
    keywords="snake camel pascal train kebab case",
    platforms='OS Independent',
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Environment :: Web Environment",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: BSD License",
        "Natural Language :: English",
        "Operating System :: OS Independent",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Text Processing :: General",
    ],
    setup_requires=setup_requires,
    rust_extensions=[RustExtension("rscase.rscase", 'Cargo.toml', binding=Binding.PyO3)],
    zip_safe=False,
)
