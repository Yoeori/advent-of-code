lines = map(l -> map(n -> parse(UInt32, n), split(l, "")), split(open(f->read(f, String), "day3.inp"), "\n"))

amount = length(lines)
gamma = 0
epsilon = 0
count = reduce(+, lines)
for c in count
    global gamma = gamma << 1
    global epsilon = epsilon << 1
    if c > (amount / 2)
        global gamma += 1
    else
        global epsilon += 1
    end
end

println("Exercise 1: ", gamma * epsilon)

possible_gammas = lines

# Find most like epsilo and gamma
bin_length = length(lines[1])
for i in 1:bin_length
    gamma_count = reduce(+, possible_gammas)
    if gamma_count[i] >= length(possible_gammas) / 2
        global possible_gammas = filter(bin -> bin[i] == 1, possible_gammas)
    else
        global possible_gammas = filter(bin -> bin[i] == 0, possible_gammas)
    end

    if length(possible_gammas) == 1
        break
    end
end

possible_epsilons = lines

for i in 1:bin_length
    epsilon_count = reduce(+, possible_epsilons)
    if epsilon_count[i] < length(possible_epsilons) / 2
        global possible_epsilons = filter(bin -> bin[i] == 1, possible_epsilons)
    else
        global possible_epsilons = filter(bin -> bin[i] == 0, possible_epsilons)
    end

    if length(possible_epsilons) <= 1
        break
    end
end

oxygen = 0
for v in possible_gammas[1]
    global oxygen = oxygen << 1
    global oxygen += v
end

scrubber = 0
for v in possible_epsilons[1]
    global scrubber = scrubber << 1
    global scrubber += v
end

println("Exercise 2: ", oxygen * scrubber)


# Gamma rate & epsilon rate
# Power = gamma * epsilon

