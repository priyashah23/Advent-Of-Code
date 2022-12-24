converted_stacks = [[], [], [], [], [], [], [], [], []]

def reverse_the_list(this_list):
	return this_list[::-1]

def row_to_stack(stack):
	for i in range(1, len(stack) - 1, 4):
		if stack[i] != ' ':
			converted_stacks[(i + 3) // 4 - 1].append(stack[i])


def convert_each_row_to_a_stack(stacks):
	for item in stacks:
		row_to_stack(item)


def convert_instructions(instructions):
	for item in instructions:
		item = item.strip().split(' ')
		instruction = item[1:len(item):2]
		follow_instruction(instruction)


def follow_instruction(instruction):
	temp = [] 
	for x in range(int(instruction[0])):
	 	item = converted_stacks[int(instruction[1]) - 1].pop()
	 	temp.append(item) 

	for z in range(len(temp)):
	 	converted_stacks[int(instruction[2]) - 1].append(temp.pop()) 


		
file = open("input5.txt")

content = file.readlines()

file.close()

convert_each_row_to_a_stack(content[0:8])

converted_stacks = list(map(reverse_the_list, converted_stacks))


instructions = content[11:]
convert_instructions(instructions)


for x in range(len(converted_stacks)):
	print(converted_stacks[x].pop())









