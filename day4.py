with open("day4input.txt") as f:
	total = 0
	lines = f.readlines()
	for line in lines:
		temp = []
		line = line.strip()
		whole_list = line.split(',')

		first_numbers = whole_list[0].split('-')
		second_numbers = whole_list[1].split('-')

		first_list = list(range(int(first_numbers[0]), int(first_numbers[1]) + 1))
		second_list = list(range(int(second_numbers[0]), int(second_numbers[1]) + 1))

		for item in first_list:
			for item2 in second_list:
				if item == item2:
					temp.append(item)

		if temp:
			total += 1

	print(total)