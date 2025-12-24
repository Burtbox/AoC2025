using var reader = new StreamReader(@".\egIn.csv");

string line = await reader.ReadToEndAsync();
string[] rangeArray = line.Split(',');

foreach (string range in rangeArray)
{
    Console.WriteLine(range);

    string[] idArray = range.Split('-');

    foreach (string id in idArray)
    {
        Console.WriteLine(id);
        if (id[0] == id[1])
        {
            Console.WriteLine(string.Format(@"invalidId {0}", id));
        }
    }
}
