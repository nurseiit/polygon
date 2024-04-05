import math


class Vector:
	def __init__(self, x: int, y: int):
		self.x = x
		self.y = y

	def __repr__(self):
		return f'Vector({self.x!r}, {self.y!r})'

	def __add__(self, other):
		x = self.x + other.x
		y = self.y + other.y
		return Vector(x, y)

	def __mul__(self, value: int):
		return Vector(self.x * value, self.y * value)

	def __rmul__(self, value: int):
		return self * value

	def __abs__(self):
		return math.hypot(self.x, self.y)

	def __bool__(self):
		return bool(self.x or self.y)


def vector_example():
	vec1 = Vector(3, 4)
	vec2 = Vector(5, 6)
	print(vec1 + vec2)
	print(vec1 * 6)
	print(6 * vec1)
	print(abs(vec1))
	print(bool(Vector(0, 0)))
	print(bool(vec1))
