test_str = ARGV[0] || "((([]([])))(()))"

type = []

for c in test_str.chars
	case c
	when "("
		type << 0
	when ")"
		abort "unexpected )" if type.last == 1  or type.length < 1
		type.pop
	when "["
		type << 1
	when "]"
		abort "unexpected ]" if type.last == 0 or type.length < 1
		type.pop
	end
end

abort "unmatched" if type.length > 0
