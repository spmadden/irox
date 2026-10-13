# SPDX-License-Identifier: MIT
# Copyright 2026 IROX Contributors
#
"""Generate XEd25519 test vectors using independent affine, variable-time math.

Test data only: never use this script with real secret keys.
Implements https://signal.org/docs/specifications/xeddsa/ sections 2.3, 3, 5.
Run with Python 3; no third-party packages are required.
"""

from hashlib import sha512


P = 2**255 - 19
Q = 2**252 + 27742317777372353535851937790883648493
D = -121665 * pow(121666, -1, P) % P
B = (
    15112221349535400772501151409588531511454012693041857206046113283949847762202,
    4 * pow(5, -1, P) % P,
)


def add(a, b):
    x, y = a
    u, v = b
    t = D * x * y * u * v % P
    return ((x * v + y * u) * pow(1 + t, -1, P) % P,
            (y * v + x * u) * pow(1 - t, -1, P) % P)


def multiply(k, point=B):
    result = (0, 1)
    while k:
        if k & 1:
            result = add(result, point)
        point = add(point, point)
        k >>= 1
    return result


def encode(point):
    x, y = point
    return (y | ((x & 1) << 255)).to_bytes(32, "little")


def hash_scalar(data):
    return int.from_bytes(sha512(data).digest(), "little") % Q


def vector(secret, message):
    k = (int.from_bytes(secret, "little") & ((1 << 255) - 8)) | (1 << 254)
    x, y = multiply(k)
    a = (-k if x & 1 else k) % Q
    public = encode(multiply(a))
    assert public == y.to_bytes(32, "little")
    u = (1 + y) * pow(1 - y, -1, P) % P
    assert (u - 1) * pow(u + 1, -1, P) % P == y
    scalar = a.to_bytes(32, "little")
    r = hash_scalar(b"\xfe" + b"\xff" * 31 + scalar + message + bytes(range(64)))
    encoded_r = encode(multiply(r))
    h = hash_scalar(encoded_r + public + message)
    s = (r + h * a) % Q
    assert multiply(s) == add(multiply(r), multiply(h, multiply(a)))
    return x & 1, scalar, public, u.to_bytes(32, "little"), encoded_r + s.to_bytes(32, "little")


if __name__ == "__main__":
    cases = [
        ("alice", bytes.fromhex("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a"), b"XEdDSA test"),
        ("bob", bytes.fromhex("5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb"), b"XEdDSA test"),
        ("zero", bytes(32), b""),
        ("ones", b"\xff" * 32, b"XEdDSA test"),
        ("one_bytes", b"\x01" * 32, b"XEdDSA test"),
    ]
    for name, secret, message in cases:
        sign, scalar, public, montgomery, signature = vector(secret, message)
        if name == "alice":
            assert montgomery.hex() == "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a"
        if name == "bob":
            assert montgomery.hex() == "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f"
        print(name, "sign:", sign)
        for label, value in [("scalar", scalar), ("public", public), ("signature", signature)]:
            print(label + ":", value.hex())