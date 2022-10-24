from functools import cache
from itertools import starmap, product


SERIAL: int = 42

Position = complex


def square(p: Position, n: int) -> set[Position]:
    vals = range(n)
    return {p + i for i in starmap(complex, product(vals, repeat=2))}


def tot_pow_ineff(p: Position, n: int) -> int:

    return sum(map(power, square(p, n)))


@cache
def tot_pow(p: Position, n: int) -> int:
    if n <= 2:
        return tot_pow_ineff(p, n)

    deltas = [0, 1, 1j, 1 + 1j]
    sections = [tot_pow(p + d, n - 1) for d in deltas]
    part = sum(sections)
    part -= 3 * tot_pow(p + 1 + 1j, n - 2)

    k = range(1, n - 1)

    borders = (
        [complex(0, i) for i in k]
        + [complex(n - 1, i) for i in k]
        + [complex(i, 0) for i in k]
        + [complex(i, n - 1) for i in k]
    )

    borders = [i + p for i in borders]

    rest = list(map(power, borders))

    print(p, n)
    print(sections)
    print([tot_pow_ineff(p + d, n - 1) for d in deltas])
    print(tot_pow(p + 1 + 1j, n - 2))

    print(rest)
    res = part - sum(rest)
    print(res)
    print(tot_pow_ineff(p, n))
    print()

    return res


def power(p: complex) -> int:
    serial = SERIAL
    rack_id = int(round(p.real)) + 10
    power = rack_id * int(round(p.imag))
    power += serial
    power *= rack_id

    power = (power % 1000) // 100
    return power - 5


def part1():
    cells = set(starmap(complex, product(range(1, 301), repeat=2)))

    posses = {k: tot_pow_ineff(k, 3) for k in cells if (v := square(k, 3)) <= cells}
    r = max(posses, key=lambda x: posses[x])
    print(posses[r])
    return r


def part2():
    cells = set(starmap(complex, product(range(1, 301), repeat=2)))

    posses = {k: sum(map(power, v)) for k in cells if (v := square(k, 3)) <= cells}

    print(tot_pow(20 + 60j, 5))
    print(tot_pow_ineff(20 + 60j, 5))


def main():
    print(part1())
    print(part2())


if __name__ == "__main__":
    main()
