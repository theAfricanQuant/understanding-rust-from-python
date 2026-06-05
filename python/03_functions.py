"""
Lesson 03 — Functions

Python: def, parameters, defaults, *args, **kwargs, returns
Rust:   fn, typed parameters, -> return type, no defaults/varargs in this lesson
"""


# --- 1. Basic function: no args, no return ---
def greet():
    print("Hello!")


greet()


# --- 2. Function with parameters ---
def greet_person(name, greeting="Hello"):
    """A simple function with a default argument."""
    print(f"{greeting}, {name}!")


greet_person("Siseng")
greet_person("Siseng", "Good morning")
greet_person("Ricky", greeting="Hi")     # keyword args


# --- 3. Function with a return value ---
def add(a, b):
    return a + b


result = add(3, 4)
print(f"3 + 4 = {result}")


# --- 4. Multiple return values (Python: tuple) ---
def min_and_max(numbers):
    return min(numbers), max(numbers)


lo, hi = min_and_max([3, 1, 4, 1, 5, 9, 2, 6])
print(f"min={lo}, max={hi}")


# --- 5. *args and **kwargs ---
def tag(tag_name, *content, **attrs):
    """Render an HTML-ish tag. *content = positional, **attrs = keyword."""
    attr_str = " ".join(f'{k}="{v}"' for k, v in attrs.items())
    inner = " ".join(content)
    print(f"<{tag_name} {attr_str}>{inner}</{tag_name}>")


tag("a", "click me", href="https://example.com", target="_blank")


# --- 6. Type hints (Python: optional, not enforced) ---
def square(num: int) -> int:
    """Square a number. Type hints are documentation only at runtime."""
    return num * num


print(f"square(5) = {square(5)}")

# This CALLS in Python even though the type hint is `int`.
# Python's type hints are NOT enforced at runtime — the error only fires inside:
try:
    square("hi")
except TypeError as e:
    print(f"Python: caught TypeError → '{e}'")
print("  (Rust would have refused this at compile time, no runtime error needed)")


# --- 7. Functions are first-class objects ---
def apply(f, x):
    """Pass a function as an argument."""
    return f(x)


print(f"apply(square, 7) = {apply(square, 7)}")
