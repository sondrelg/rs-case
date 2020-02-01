from subprocess import call
from decouple import config

if __name__ == '__main__':
    call(['maturin', 'publish', '-o', '-u', f'{config("PYPI_USERNAME")}', '-p', f'{config("PYPI_PASSWORD")}'])