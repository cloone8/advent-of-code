class MainClass {
    private static List<Number> numbers = [];
    private static List<Gear> gears = [];
    public static void Main (string[] args) {
        int i = 0;

        foreach(var line in File.ReadLines("input.txt")) {
            ParseLine(line, i++);
        }

        foreach(var gear in gears) {
            foreach(var number in numbers) {
                if(AdjacentToSymbol(number, gear)) {
                    gear.Numbers.Add(number);
                }
            }
        }

        var sum = gears
        .Where(g => g.Numbers.Count == 2)
        .Select(g => g.Numbers[0].Value * g.Numbers[1].Value)
        .Sum();

        Console.WriteLine(sum);
    }

    private static bool AdjacentToSymbol(Number number, Gear gear) {
        for(int i = 0; i < number.Width; i++) {
            if(AdjacentToCoord(gear, number.X + i, number.Y)) {
                return true;
            }
        }

        return false;
    }

    private static bool AdjacentToCoord(Gear gear, int x, int y) {
        return gear.X >= x - 1 && gear.X <= x + 1 && gear.Y >= y - 1 && gear.Y <= y + 1;
    }

    private static void ParseLine(string line, int lineNumber) {
        int i = 0;

        while(i < line.Length) {
            char c = line[i];

            if(c == '.') {
                i++;
                continue;
            }

            if(c >= '0' && c <= '9') {
                Number n = ParseNumber(line, i, lineNumber);

                numbers.Add(n);
                i += n.Width;
                continue;
            }

            if(c == '*') {
                gears.Add(new Gear(i, lineNumber));
                i++;
                continue;
            }

            i++;
        }
    }

    private static Number ParseNumber(string line, int x, int y) {
        int width = 0;
        string number = "";

        while((x + width) < line.Length) {
            char c = line[x + width];

            if(c >= '0' && c <= '9') {
                number += c;
                width++;
                continue;
            }

            break;
        }

        return new Number(int.Parse(number), x, y, width);
    }
}

class Number(int value, int x, int y, int width) {
    public int Value { get; set; } = value;
    public int X { get; set; } = x;
    public int Y { get; set; } = y;
    public int Width { get; set; } = width;
}

class Gear(int x, int y) {
    public List<Number> Numbers { get; set; } = [];
    public int X { get; set; } = x;
    public int Y { get; set; } = y;
}
