class BitBoard:
    def __init__(self, val: int = 0):
        self.val: int = val

    def lsb(self) -> int:
        return (self.val & -self.val).bit_length() - 1

    def pop_lsb(self) -> int:
        lsb: int = self.lsb()
        self.val ^= (1 << lsb)
        return lsb

    def set_squares(self, square: int):
        self.val |= (1 << square)

    def remove_square(self, square: int):
        self.val &= ~(1 << square)

    def toggle_squares(self, square: int):
        self.val ^= (1 << square)

    def count_ones(self):
        self.val.bit_count()

    def __str__(self):
        return bin(self.val)[2:].zfill(64)

    def __repr__(self):
        return str(self)

    def __or__(self, other):
        return BitBoard(self.val | other.val)

    def __and__(self, other):
        return BitBoard(self.val & other.val)

    def __xor__(self, other):
        return BitBoard(self.val ^ other.val)

    def __invert__(self):
        return BitBoard(~self.val)


if __name__ == '__main__':
    bb = BitBoard(0b0010010000)


