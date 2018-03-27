def pow_mod(x, y, z):
    "Calculate (x ** y) % z efficiently."
    number = 1
    while y:
        if y & 1:
            number = number * x % z
        y >>= 1
        x = x * x % z
    return number



i = 80000000
num = 0
while (num != 10483580695461280548150531):
    num = pow_mod(7, i, 15045919506432000000000001)
    i += 1
    if (i % 1000000 == 0):
        print(i)
print(i)