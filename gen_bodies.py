from random import randint

min_x = -10000
max_x = 10000
min_y = -10000
max_y = 10000
min_r = 50
max_r = 250

for x in range(100):
	print("	Body {")
	print("		pos: Vector{")
	print(f"			x: {randint(min_x, max_x)}.0,")
	print(f"			y: {randint(min_y, max_y)}.0,")
	print("		},")
	print(f"		radius: {randint(min_r, max_r)}.0")
	print("	},")
