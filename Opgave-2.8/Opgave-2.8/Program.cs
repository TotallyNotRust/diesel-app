// See https://aka.ms/new-console-template for more information
using Microsoft.EntityFrameworkCore;
using Opgave_2._8;

Console.WriteLine("Hello, World!");

using (var db = new databaseContext())
{
    Console.WriteLine(db.Database.EnsureCreated());

    var groups = db.Groups.ToList();

    Console.WriteLine(groups);

    foreach(var group in groups)
    {
        Console.WriteLine(group.Name);
    }
}