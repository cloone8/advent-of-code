using System.Data;

class MainClass {
    public static void Main(string[] args) {
        int result = File
            .ReadLines("input.txt")
            .Select(GetCalibrationValue)
            .Sum();

        Console.WriteLine(result);
    }

    private static int GetCalibrationValue(string line) {
        List<char> onlyDigits = line.Where(c => c >= '0' && c <= '9').ToList();

        if(onlyDigits.Count == 0) {
            return 0;
        }

        string firstLast = onlyDigits.First().ToString() + onlyDigits.Last().ToString();

        return int.Parse(firstLast);
    }
}
