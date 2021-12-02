numbers = map(l -> parse(UInt32, l), split(open(f->read(f, String), "day1.inp"), "\n"))

prev_n = typemax(UInt32)
n_count = 0

for n in numbers
    if n > prev_n
        global n_count += 1
    end
    global prev_n = n
end

println("Exercise 1: ", n_count)


prev_n = typemax(UInt32)
n_count = 0

for i in 1:(length(numbers)-2)
    n = numbers[i] + numbers[i+1] + numbers[i+2]
    if n > prev_n
        global n_count += 1
    end
    global prev_n = n
end

println("Exercise 2: ", n_count)
