using System.Runtime.CompilerServices;

class MainClass {
    private static List<Card> cards = [];

    public static void Main(string[] args) {
        cards = File
            .ReadLines("input.txt")
            .Select(ParseCard)
            .ToList();

        for(int i = 0; i < cards.Count; i++) {
            int matching = GetNumMatching(cards[i]);

            for(int j = 1; j < cards.Count && j <= matching; j++) {
                cards[i + j].Copies += cards[i].Copies;
            }
        }

        int sum = cards
            .Select(c => c.Copies)
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
            YourNumbers = yourNumbersList,
            Copies = 1
        };
    }

    private static int GetNumMatching(Card card) {
        int matching = 0;

        foreach (int number in card.YourNumbers) {
            if (card.WinningNumbers.Contains(number)) {
                matching++;
            }
        }

        return matching;
    }
}

class Card {
    public int Copies = 1;
    public List<int> WinningNumbers = [];
    public List<int> YourNumbers = [];

    public Card() {}
}
