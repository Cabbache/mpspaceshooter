from random import randint

min_x = -5000
max_x = 5000
min_y = -5000
max_y = 5000
min_r = 50
max_r = 150

for x in range(25):
	print("	Body {")
	print("		pos: Vector{")
	print(f"			x: {randint(min_x, max_x)}.0,")
	print(f"			y: {randint(min_y, max_y)}.0,")
	print("		},")
	print(f"		radius: {randint(min_r, max_r)}.0")
	print("	},")
