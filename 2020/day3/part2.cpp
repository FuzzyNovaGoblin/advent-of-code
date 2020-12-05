#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

struct TreeMap {
   vector<string> strVec;

   char getAt(int x, int y) const {
      int firstLen = strVec[0].length();

      while(x >= firstLen)
         x -= firstLen;

      return strVec[y][x];
   }
};
int countTrees(const TreeMap& ourMap, int xSlope, int ySlope);

int main() {
   TreeMap ourMap;
   ifstream inFile;

   inFile.open("input.txt");
   string buff;
   while(getline(inFile, buff)) {
      ourMap.strVec.push_back(buff);
   }
   inFile.close();

   long int treeCount = 1;
   treeCount *= countTrees(ourMap, 1, 1);
   treeCount *= countTrees(ourMap, 3, 1);
   treeCount *= countTrees(ourMap, 5, 1);
   treeCount *= countTrees(ourMap, 7, 1);
   treeCount *= countTrees(ourMap, 1, 2);
   cout << treeCount << endl;

}

int countTrees(const TreeMap& ourMap, int xSlope, int ySlope) {
   int treeCount = 0;

   for(int x = 0, y = 0; y < ourMap.strVec.size(); x += xSlope, y += ySlope) {
      treeCount += ourMap.getAt(x, y) == '#';
   }
   return treeCount;
}