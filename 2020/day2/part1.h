#if !defined(PART1_H)
#define PART1_H
#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

namespace part1
{
struct Entry {
   int mMin;
   int mMax;
   char mVal;
   string mStr;
   Entry(int min, int max, char val, string str) {
      mMin = min;
      mMax = max;
      mVal = val;
      mStr = str;
   }
   bool isValid() {
      int c = 0;
      for(int i = 0; i < mStr.length(); i++) {
         if(mStr[i] == mVal) c++;
      }
      return c <= mMax && c >= mMin;
   }
};

int main(int argc, char const *argv[]) {
   ifstream inFile("input.txt");
   vector<Entry *> entries;

   {
      int i, k;
      string buff;

      while(getline(inFile, buff)) {
         for(i = 0; i < buff.length(); i++) {
            if(buff[i] == '-')
               break;
         }

         for(k = i + 1; k < buff.length(); k++) {
            if(buff[k] == ' ')
               break;
         }
         entries.push_back(new Entry(stoi(buff.substr(0, i)),
                                     stoi(buff.substr(i + 1, k - i - 1)),
                                     buff[k + 1],
                                     buff.substr(k + 4)));
      }
   }
   inFile.close();

   int validCount = 0;
   for(int i = 0; i < entries.size(); i++) {
      if(entries[i]->isValid()) {
         validCount++;
      }
   }
   for(int i = 0; i < entries.size(); i++) {
      delete entries[i];
   }
   entries.clear();

   cout << "valid entries: " << validCount << endl;
   return 0;
}
}  // namespace part1

#endif