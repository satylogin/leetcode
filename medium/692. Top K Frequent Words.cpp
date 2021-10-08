class Solution {
public:
    static bool comp(pair<int,string>&p1,pair<int,string>&p2){
        if(p1.first>p2.first){
            return true;
        }
        else if(p1.first==p2.first){
            if(p1.second<p2.second){
                return true;
            }
            else{
                return false;
            }
        }
        else{
            return false;
        }
    }
    vector<string> topKFrequent(vector<string>& words, int k) {
        int i,n=words.size();
        map<string,int>mp;
        for(i=0;i<n;i++){
            mp[words[i]]++;
        }
        vector<pair<int,string>>temp;
        for(auto it=mp.begin();it!=mp.end();it++){
            temp.push_back({it->second,it->first});
        }
        sort(temp.begin(),temp.end(),comp);
        vector<string>ans;
        i=0;
        while(k--){
            ans.push_back(temp[i].second);
            i++;
        }
        return ans;
    }
};
