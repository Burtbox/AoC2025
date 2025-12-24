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


long runningTotalB = 0L;
foreach (string range in rangeArray)
{
    string[] idArray = range.Split('-');
    long lowerBound = long.Parse(idArray[0]);
    long upperBound = long.Parse(idArray[1]);

    for (long i = lowerBound; i <= upperBound; i++)
    {
        string id = i.ToString();
        for (int segmentLength = 1; segmentLength * 2 <= id.Length; segmentLength++)
        {
            if (id.Length % segmentLength != 0)
            {
                continue;
            }
            bool isRepetitive = true;
            string initialSegment = id[..segmentLength];
            for (int pos = segmentLength; pos < id.Length && isRepetitive; pos += segmentLength)
            {
                isRepetitive = initialSegment == id.Substring(pos, segmentLength);
            }
            if (isRepetitive)
            {
                runningTotalB += i;
                break;
            }
        }
    }
}

Console.WriteLine(runningTotalB);