using System;

namespace discriminant_csharp
{
    class Program
    {
        public static void Main(String[] args)
        {
            while (true)
            {
                Console.WriteLine("Discriminant Calculator");
                Console.WriteLine("Enter values according to the sample: ax^2+bx+c");

                // Gets variables and if it is empty gives it null value
                Console.Write("Enter variable a: ");
                double? a = Console.ReadLine() switch
                {
                    string s when s.Length > 0 => Convert.ToDouble(s),
                    _ => null
                };

                Console.Write("Enter variable b: ");
                double? b = Console.ReadLine() switch
                {
                    string s when s.Length > 0 => Convert.ToDouble(s),
                    _ => null
                };

                Console.Write("Enter variable c: ");
                double? c = Console.ReadLine() switch
                {
                    string s when s.Length > 0 => Convert.ToDouble(s),
                    _ => null
                };
                
                try
                {
                    // Calculates discriminant
                    (double x1, double? x2) = Discriminant.calc_discriminant(a, b, c);
                    if (x2.HasValue)
                    {
                        Console.WriteLine($"Answer: x1={x1}, x2={x2}\n");
                    }
                    else
                    {
                        Console.WriteLine($"Answer: x={x1}\n");
                    }
                }
                catch (Exception e)
                {
                    Console.WriteLine($"Error: {e}\n");
                }
            }
        }
    }
}