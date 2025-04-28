import random

# Generate 4 random float values between 0 and 1
values = [round(random.uniform(0, 1), 4) for _ in range(4)]

# Join them with semicolons and print
print(";".join(map(str, values)))

