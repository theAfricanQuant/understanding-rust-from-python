"""
Lesson 04 — If/else

Python: if / elif / else — statements, truthy/falsy values allowed
Rust:   if / else if / else — expressions, must be bool, all branches same type
"""


# --- 1. Basic if/else: it's a STATEMENT in Python ---
def bigger(a, b):
    if a > b:
        return a
    else:
        return b
    # Could also write: return a if a > b else b  (ternary)


print(f"bigger(10, 8) = {bigger(10, 8)}")
print(f"bigger(32, 42) = {bigger(32, 42)}")
print(f"bigger(42, 42) = {bigger(42, 42)}")


# --- 2. Truthy / falsy — Python's "feature" ---
def describe(n):
    if n:                       # ANY non-zero, non-empty value is truthy
        return "truthy"
    else:
        return "falsy"


for n in [0, 1, -1, "", "hi", [], [0], None]:
    print(f"describe({n!r}) = {describe(n)}")


# --- 3. elif chains ---
def grade(score):
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"


print(f"grade(95) = {grade(95)}")
print(f"grade(72) = {grade(72)}")
print(f"grade(50) = {grade(50)}")


# --- 4. Ternary expression ---
def sign(n):
    return "positive" if n > 0 else "non-positive"


print(f"sign(5)  = {sign(5)}")
print(f"sign(-3) = {sign(-3)}")
print(f"sign(0)  = {sign(0)}")


# --- 5. Nested if ---
def describe_number(n):
    if n != 0:
        if n > 0:
            return "positive non-zero"
        else:
            return "negative non-zero"
    else:
        return "zero"


for n in [5, -3, 0]:
    print(f"describe_number({n}) = {describe_number(n)}")


# --- 6. match-case (Python 3.10+) ---
def http_status(code):
    match code:
        case 200:
            return "OK"
        case 301 | 302:
            return "Redirect"
        case 404:
            return "Not Found"
        case 500:
            return "Server Error"
        case _:
            return "Unknown"


for c in [200, 302, 404, 418]:
    print(f"http_status({c}) = {http_status(c)}")
