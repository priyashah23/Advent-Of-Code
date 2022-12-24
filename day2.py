total = 0
conversion = [{'X': 3, 'Y': 1, 'Z': 2}, {'X': 1, 'Y': 2, 'Z': 3}, {'X': 2, 'Y': 3, 'Z': 1}]

the_score_for_when_you_win = {'X': 0, 'Y':3, 'Z':6}

with open("day2input.txt") as f:
	lines = f.readlines()
	for line in lines:
		if line[0] == 'A':
			total += conversion[0][line[2]] + the_score_for_when_you_win[line[2]]
		elif line[0] == 'B':
			total += conversion[1][line[2]] + the_score_for_when_you_win[line[2]]
		elif line[0] == 'C':
			total += conversion[2][line[2]] + the_score_for_when_you_win[line[2]]
print(total)
