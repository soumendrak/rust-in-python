import librust_exceptions

print(librust_exceptions.divide(10, 1))

try:
    print(librust_exceptions.divide(10, 0))
except ZeroDivisionError:
    print("Cannot divide by zero")