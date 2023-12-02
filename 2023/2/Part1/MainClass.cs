public class MainClass {
    private static readonly int maxRed = 12;
    private static readonly int maxGreen = 13;
    private static readonly int maxBlue = 14;

    public static void Main(string[] args) {
        int sum = File.ReadLines("input.txt")
            .ToList()
            .Select(ParseGame)
            .Where(GameIsValid)
            .Select(g => g.Id)
            .Sum();

        Console.WriteLine("Sum: " + sum);
    }

    private static Game ParseGame(string line) {
        int id = int.Parse(line.Replace("Game ", "").Split(':')[0]);

        Game game = new() { Id = id };

        List<Round> rounds = line
            .Split(':')[1]
            .Split(';')
            .Select(s => s.Trim())
            .Select(ParseRound)
            .ToList();

        game.Rounds.AddRange(rounds);

        return game;
    }

    private static Round ParseRound(string line) {
        Round round = new() { Red = 0, Green = 0, Blue = 0 };

        List<string> grabbed = line.Split(',').Select(s => s.Trim()).ToList();

        foreach(var s in grabbed) {
            if(s.Contains("red")) {
                round.Red = int.Parse(s.Replace("red", "").Trim());
            } else if(s.Contains("green")) {
                round.Green = int.Parse(s.Replace("green", "").Trim());
            } else if(s.Contains("blue")) {
                round.Blue = int.Parse(s.Replace("blue", "").Trim());
            } else {
                throw new Exception("Unknown color: " + s);
            }
        }

        return round;
    }

    private static bool GameIsValid(Game game) {
        return game.Rounds.All(RoundIsValid);
    }

    private static bool RoundIsValid(Round round) {
        return round.Red <= maxRed && round.Green <= maxGreen && round.Blue <= maxBlue;
    }
}

struct Game {
    public Game() {}

    public int Id { get; set; }

    public List<Round> Rounds { get; } = [];
}

struct Round {
    public int Red { get; set; }
    public int Green { get; set;}
    public int Blue { get; set; }
}
