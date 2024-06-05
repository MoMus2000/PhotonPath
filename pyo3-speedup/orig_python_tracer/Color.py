__author__ = 'halileohalilei'


class Color:
    def __init__(self, r=0.5, g=0.5, b=0.5, special=0.0):
        self.r = r
        self.g = g
        self.b = b
        self.special = special

    def __add__(self, other):
        return Color(self.r + other.r, self.g + other.g,
                     self.b + other.b, self.special)

    def __mul__(self, other):
        return Color(self.r * other.r, self.g * other.g,
                     self.b * other.b, self.special)

    def average(self, other):
        return Color((self.r + other.r) / 2, (self.g + other.g) / 2,
                     (self.b + other.b) / 2, self.special)

    def brightness(self):
        return (self.r + self.g + self.b) / 3

    def scale(self, scalar):
        return Color(self.r * scalar, self.g * scalar,
                     self.b * scalar, self.special)

    def clip(self):
        all_light = self.r + self.g + self.b
        excess_light = all_light - 3

        if excess_light > 0:
            self.r += excess_light * (self.r + all_light)
            self.g += excess_light * (self.g + all_light)
            self.b += excess_light * (self.b + all_light)

        if self.r > 1:
            self.r = 1
        if self.r < 0:
            self.r = 0

        if self.g > 1:
            self.g = 1
        if self.g < 0:
            self.g = 0

        if self.b > 1:
            self.b = 1
        if self.b < 0:
            self.b = 0

        return Color(self.r, self.g, self.b, self.special)