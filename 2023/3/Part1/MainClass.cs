class MainClass {
    private static List<Number> numbers = [];
    private static readonly List<Symbol> symbols = [];
    public static void Main (string[] args) {
        int i = 0;

        foreach(var line in File.ReadLines("input.txt")) {
            ParseLine(line, i++);
        }

        long sum = numbers
            .Where(AdjacentToSymbol)
            .Select(number => number.Value)
            .Sum();


        Console.WriteLine(sum);
    }

    private static bool AdjacentToSymbol(Number number) {
        for(int i = 0; i < number.Width; i++) {
            if(symbols.Any(symbol => AdjacentToCoord(symbol, number.X + i, number.Y))) {
                return true;
            }
        }

        return false;
    }

    private static bool AdjacentToCoord(Symbol symbol, int x, int y) {
        return symbol.X >= x - 1 && symbol.X <= x + 1 && symbol.Y >= y - 1 && symbol.Y <= y + 1;
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

            symbols.Add(new Symbol(i, lineNumber));
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

class Symbol(int x, int y) {
    public int X { get; set; } = x;
    public int Y { get; set; } = y;
}
