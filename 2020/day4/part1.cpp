#include <fstream>
#include <iostream>
#include <map>
#include <vector>

using namespace std;

int main() {
   ifstream inFile("input.txt");

   vector<map<string, string>> pssPts;
   pssPts.push_back({});

   string buff;
   while(getline(inFile, buff)) {
      if(buff == "") {
         // cout << "bad\n";
         pssPts.push_back({});
      }
      else {
         vector<int> pts;
         for(size_t i = 0; i < buff.length(); i++) {
            if(buff[i] == ':' || buff[i] == ' ') {
               pts.push_back(i);
            }
         }
         pts.push_back(buff.length());
         // cout << buff.substr(0, pts[0]) << ":" << buff.substr(pts[0] + 1, pts[1]-1 - pts[0] + 1) << endl;
         for (size_t i = 0; i < pts.size(); i++)
         {
         cout << pts[i] << " ";
         }
         cout << endl;

         pssPts.back()[buff.substr(0, pts[0])] = buff.substr(pts[0] + 1, pts[1] - (pts[0] + 1));
         for(int i = 2; i < pts.size(); i += 2) {
            pssPts.back()[buff.substr(pts[i - 1] + 1, (pts[i] - 1) - (pts[i - 1] + 1)+1)] = buff.substr((pts[i] + 1), (pts[i + 1]) - (pts[i] + 1)+1);
         }
      }
   }
   inFile.close();
   string reqs[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};

   int count = 0;
   int count2 = 0;
   cout << pssPts.size() << endl;
   for(size_t i = 0; i < pssPts.size(); i++) {
      bool tmp = true;
      for(size_t k = 0; k < 7; k++) {
         if(pssPts[i][reqs[k]] == "") {
            cout << "here: " << reqs[k] <<"   " << pssPts[i][reqs[k]] << endl;
            count++;
            break;
         }
      }


      cout << pssPts[i].size() << endl;
      for(std::map<string, string>::iterator it = ((pssPts[i]).begin()); it != pssPts[i].end(); ++it)
         std::cout << "|" << it->first << "| => |" << it->second << "|\n";
   }
   cout << count << endl;
}