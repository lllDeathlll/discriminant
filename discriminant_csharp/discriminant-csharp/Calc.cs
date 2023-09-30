using System;
using System.Collections.Generic;
using System.Diagnostics.Metrics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace discriminant_csharp
{
    public class Discriminant
    {
        public static (double, double?) calc_discriminant(double? a_opt, double? b_opt, double? c_opt)
        {
            // Sets default value of 0 if variable is null
            double a = a_opt ?? 0;
            double b = b_opt ?? 0;
            double c = c_opt ?? 0;

            // If two or more variables are null throws exception
            if (a == 0 && b == 0 || b == 0 && c == 0 || a == 0 && c == 0)
            {
                throw new Exception("You must enter 2 or more variables");
            }

            else if (a == 0)
            {
                double x1 = c / b * -1;
                return (x1, null);
            }

            else if (b == 0)
            {
                double x1 = Math.Sqrt(a / c * -1);
                double x2 = x1 * -1;
                return (x1, x2);
            }

            else if (c == 0)
            {
                double x1 = b / a * -1;
                double x2 = 0;
                return (x1, x2);
            }

            else
            {
                // Calculates discriminant
                double discriminant = Math.Pow(b, 2.0) - 4 * a * c;
                // Throws exception if discriminant is negative
                if (discriminant < 0)
                {
                    throw new Exception("Discriminant is negative");
                }
                // Calculates x's
                else
                {
                    double x1 = (b * -1 + Math.Sqrt(discriminant)) / 2 * a;
                    double x2 = (b * -1 - Math.Sqrt(discriminant)) / 2 * a;
                    return (x1, x2);
                }
            }
        }
    }
}
