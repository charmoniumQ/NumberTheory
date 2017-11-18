// clang++ -O3 main.cpp && ./a.out && head results.txt
#include <fstream>
#include <iomanip>

using namespace std;

const string filename = "results.txt";

int **triangle;
/*
   m = triangle[b][a] determines for which k, p^a k-ary divides p^b
   If m is even, then p^a k-ary divides p^b for all even k < m
   If m is odd, then p^a does not k-ary divide p^b for all odd k < m
*/

void calculate (int rows);
bool kDivides(int b, int a, int k);
bool order(int, int);
void printTriangle(int rows);
void printMobius(int rows);

int main ()
{
   calculate(256);
   printMobius(10);
   return 0;
}


void calculate (int rows)
{
   triangle = new int*[rows + 1];
   triangle[0] = new int[1];
   triangle[0][0] = 1;
   for (int b = 1; b <= rows; b++)
   {
      int* row = new int[b + 1];
      row[0] = 1;
      row[b] = 1;
      for (int a = 1; a < b; a++)
      {
         if ((a & b) == a) // p^a infinitarily divides p^b, so m is odd
         {
            int lb = 1;
            int ub = b;
            if (ub % 2 == 0)
               ub--;
            int m;
            while (lb != ub)
            {
               m = (lb + ub + 2) / 4 * 2 - 1;
               if (kDivides(a, b, m))
                  ub = m;
               else
                  lb = m + 2;
            }
            row[a] = lb;
         }
         else            // m is even
         {
            int lb = 2;
            int ub = b;
            if (ub % 2 == 1)
               ub --;
            int m;
            while (lb != ub)
            {
               m =  (lb + ub) / 4 * 2;
               if (kDivides(a, b, m))
                  lb = m + 2;
               else
                  ub = m;
            }
            row[a] = lb;
         }
      }
      triangle[b] = row;
   }
}

/*
   true if p^a k-ary divides p^b
*/
bool kDivides (int a, int b, int k)
{
   for (int i = 1; i <= a && i <= b - a; i++)
   {
      if (order(triangle[a][i], k - 1) && order(triangle[b - a][i], k - 1))
         return false;
   }
   return true;
}

/*
   true if the number k1 on the characteristic triangle indicates that d k2-ary divides n
*/
bool order (int k1, int k2)
{
   if (k1 % 2 == 0)
      return k2 % 2 == 0 && k1 > k2;
   else
      return k2 % 2 == 0 || k1 <= k2;
}

void printTriangle (int rows)
{
   ofstream fout("results.txt");
   for (int b = 0; b <= rows; b++)
   {
      for (int a = 0; a <= b; a++)
         fout << triangle[b][a] << "\t";
      fout << endl;
   }
   fout.close();
}

void printMobius(int maxk) {
  ofstream fout("results.txt");
  for (int k = 0; k <= maxk; ++k) {
	int* kmobius = new int[maxk+1]();
	kmobius[0] = 1;
	fout << kmobius[0] << ",";
	for (int n = 1; n <= maxk; ++n) {
	  int sum = 0;
	  for (int i = 0; i < n; ++i) {
		if (kDivides(i, n, k)) {
		  sum += kmobius[i];
		}
	  }
	  kmobius[n] = -sum;
	  fout << kmobius[n] << ",";
	}
	fout << "\n";
  }
}
