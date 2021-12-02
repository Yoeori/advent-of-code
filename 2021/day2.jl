instructions = map(l -> split(l, " "), split(open(f->read(f, String), "day2.inp"), "\n"))

depth = 0
pos = 0

for (instruction, x) in instructions
    if instruction == "forward"
        global pos += parse(UInt32, x)
    elseif instruction == "down"
        global depth += parse(UInt32, x)
    elseif instruction == "up"
        global depth -= parse(UInt32, x)
    end
end

println("Exercise 1: ", depth * pos)

depth = 0
pos = 0
aim = 0

for (instruction, x) in instructions
    x = parse(UInt32, x)
    if instruction == "forward"
        global pos += x
        global depth += aim * x
    elseif instruction == "down"
        global aim += x
    elseif instruction == "up"
        global aim -= x
    end
end

println("Exercise 2: ", depth * pos)
