#!/usr/bin/env python3
"""
This is a comprehensive Python file demonstrating various syntax features.
It includes a multiline docstring.
"""

from typing import List, Dict, Optional, Union
import sys
from dataclasses import dataclass
from enum import Enum, auto
import asyncio

# Different number literals
INTEGER = 42
FLOAT = 3.14159
COMPLEX = 3 + 4j
HEX = 0xFF
OCTAL = 0o755
BINARY = 0b1010
UNDERSCORED_NUM = 1_000_000

# String literals
SINGLE_QUOTE = 'Hello'
DOUBLE_QUOTE = "World"
TRIPLE_SINGLE = '''This is a
multiline string'''
TRIPLE_DOUBLE = """Another
multiline string"""
F_STRING = f"The answer is {INTEGER}"
RAW_STRING = r"\raw\string"
BYTES = b"bytes"
UNICODE = u"unicode"

# Collection literals
LIST = [1, 2, 3]
TUPLE = (1, 2, 3)
SET = {1, 2, 3}
DICT = {"a": 1, "b": 2}
LIST_COMP = [x * 2 for x in range(5)]
DICT_COMP = {str(x): x for x in range(3)}
SET_COMP = {x % 3 for x in range(10)}
GENERATOR = (x ** 2 for x in range(5))

class Color(Enum):
    RED = auto()
    GREEN = auto()
    BLUE = auto()

@dataclass
class Point:
    x: float
    y: float
    label: Optional[str] = None

    def __post_init__(self):
        self.magnitude = (self.x ** 2 + self.y ** 2) ** 0.5

    @property
    def quadrant(self) -> int:
        if self.x >= 0 and self.y >= 0: return 1
        elif self.x < 0 and self.y >= 0: return 2
        elif self.x < 0 and self.y < 0: return 3
        else: return 4

    def __str__(self) -> str:
        return f"Point({self.x}, {self.y})"

async def async_function() -> None:
    await asyncio.sleep(0.1)
    print("Async operation complete")

def decorator(func):
    def wrapper(*args, **kwargs):
        print("Before function call")
        result = func(*args, **kwargs)
        print("After function call")
        return result
    return wrapper

@decorator
def example_function(x: int, *, keyword_only: str = "default") -> str:
    """Function with type hints and keyword-only arguments."""
    match x:
        case 0:
            return "zero"
        case 1:
            return "one"
        case _:
            return "many"

try:
    result = 1 / 0
except ZeroDivisionError as e:
    print(f"Caught error: {e}")
finally:
    print("Cleanup")

if __name__ == "__main__":
    # Walrus operator
    if (n := len(sys.argv)) > 1:
        print(f"Got {n} arguments")
    
    # Context manager
    with open(__file__, "r") as f:
        content = f.read()
    
    # Lambda function
    square = lambda x: x * x
    
    # Assert statement
    assert square(2) == 4, "Square function failed"
