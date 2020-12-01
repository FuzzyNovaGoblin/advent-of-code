#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main()
{
   fstream inFile("input.txt");
   char *buff = new char[200];
   vector<int> vals;

   while (inFile.getline(buff, 200))
   {
      vals.push_back(stoi(buff));
   }
   size_t i, j, k;
   for (i = 0; i < vals.size(); i++)
   {
      for (j = i + 1; j < vals.size(); j++)
      {
         for (k = j + 1; k < vals.size(); k++)
         {
            if (vals[i] + vals[j] + vals[k] == 2020)
            {
               goto end;
            }
         }
      }
   }

end:
   cout << "1: " << vals[i] << "\t2: " << vals[j] << "\t3: " << vals[k] << "\t p: " << vals[i] * vals[j] * vals[k] << endl;
   delete[] buff;
}