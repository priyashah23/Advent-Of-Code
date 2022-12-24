
calorie_list = []


with open("input.txt") as f:
	lines = f.readlines()
	total = 0
	for line in lines:
		line = line.strip()
		if line == "":
			calorie_list.append(total)
			total = 0
			continue
		total +=  int(line)

calorie_list.sort(reverse=True)
print(calorie_list[0] + calorie_list[1] + calorie_list[2])

