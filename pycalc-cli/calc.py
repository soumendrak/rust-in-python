"""
PyCalc CLI using Python Fire
"""

import fire
from libpycalc_cli import (
    add_as_string,
    subtract_as_string,
    multiply_as_string,
    divide_as_string,
)


class Calculator:
    """
    Rust based Calculator
    """

    def add(self, a: int, b: int) -> str:
        """
        Adds two numbers
        """
        return add_as_string(a, b)

    def subtract(self, a: int, b: int) -> str:
        """
        Subtracts two numbers
        """
        return subtract_as_string(a, b)

    def multiply(self, a: int, b: int) -> str:
        """
        Multiplies two numbers
        """
        return multiply_as_string(a, b)

    def divide(self, a: int, b: int) -> str:
        """
        Divides two numbers
        """
        return divide_as_string(a, b)


if __name__ == "__main__":
    fire.Fire(Calculator)