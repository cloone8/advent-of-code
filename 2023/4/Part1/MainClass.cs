class MainClass {
    public static void Main(string[] args) {
        int sum = File
            .ReadLines("input.txt")
            .Select(ParseCard)
            .Select(GetPoints)
            .Sum();

        Console.WriteLine(sum);
    }

    private static Card ParseCard(string line) {
        string noId = line.Split(':')[1].Trim();
        string[] numbers = noId.Split('|');
        string winningNumbers = numbers[0].Trim();
        string yourNumbers = numbers[1].Trim();
        List<int> winningNumbersList = winningNumbers.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToList();
        List<int> yourNumbersList = yourNumbers.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToList();

        return new Card() {
            WinningNumbers = winningNumbersList,
            YourNumbers = yourNumbersList
        };
    }

    private static int GetPoints(Card card) {
        int points = 0;
        foreach (int number in card.YourNumbers) {
            if (card.WinningNumbers.Contains(number)) {
                if(points == 0) {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        return points;
    }
}

struct Card {
    public List<int> WinningNumbers = [];
    public List<int> YourNumbers = [];

    public Card() {}
}
