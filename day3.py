total = 0

def find_common_letter(theselines):
 	for char in theselines[0]:
 		for char2 in theselines[1]:
 			for char3 in theselines[2]:
	 			if char == char2 == char3:
	 				return char

def calculate_points_for_letter(common_letter) -> int:
	if common_letter.islower():
		return ord(common_letter) - 96
	else:
		return ord(common_letter) - 38

with open("day3input.txt") as f:
 	lines = f.readlines()
 	for i in range(100):
 		theselines = []
	 	for x in range(i * 3, (i * 3) + 3):
	 		theselines.append(lines[x].strip())
	 	common_letter = find_common_letter(theselines)
	 	total += calculate_points_for_letter(common_letter)
print(total)
