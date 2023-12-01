using System.Data;

class MainClass {
    private static Dictionary<string, char> digitsAsStrings = new Dictionary<string, char> {
        {"one", '1'},
        {"two", '2'},
        {"three", '3'},
        {"four", '4'},
        {"five", '5'},
        {"six", '6'},
        {"seven", '7'},
        {"eight", '8'},
        {"nine", '9'},
    };

    public static void Main(string[] args) {
        int result = File
            .ReadLines("input.txt")
            .Select(GetCalibrationValue)
            .Sum();

        Console.WriteLine(result);
    }

    private static void SetDigits(ref char firstDigit, ref char lastDigit, char newDigit) {
        if(firstDigit == '\0') {
            firstDigit = newDigit;
        }

        lastDigit = newDigit;
    }

    private static int GetCalibrationValue(string line) {
        char firstDigit = '\0';
        char lastDigit = '\0';

        for(int i = 0; i < line.Length; i++) {
            if(line[i] >= '0' && line[i] <= '9') {
                SetDigits(ref firstDigit, ref lastDigit, line[i]);
                continue;
            }

            foreach(KeyValuePair<string, char> digit in digitsAsStrings) {
                if(line[i..].StartsWith(digit.Key)) {
                    SetDigits(ref firstDigit, ref lastDigit, digit.Value);
                    break;
                }
            }
        }

        return int.Parse(firstDigit.ToString() + lastDigit.ToString());
    }
}
