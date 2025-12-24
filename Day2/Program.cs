using System.Diagnostics;

using var reader = new StreamReader(@".\in.csv");

string line = await reader.ReadToEndAsync();
string[] rangeArray = line.Split(',');
long runningTotal = 0L;

foreach (string range in rangeArray)
{
    // Console.WriteLine(range);

    string[] idArray = range.Split('-');
    long lowerBound = long.Parse(idArray[0]);
    long upperBound = long.Parse(idArray[1]);

    for (long i = lowerBound; i <= upperBound; i++)
    {
        string id = i.ToString();
        if (id.Length % 2 != 0)
        {
            continue;
        }
        int halfLength = id.Length / 2;
        if (id.AsSpan(0, halfLength).SequenceEqual(id.AsSpan(halfLength, halfLength)))
        {
            runningTotal += i;
        }
    }

}

Console.WriteLine(runningTotal);