class Solution {
public:
    int findPairs(vector<int>& nums, int k) {
        int i,j,n=nums.size();
        set<pair<int,int>>s;
        sort(nums.begin(),nums.end());
        i=0;
        j=i+1;
        while(j<n&&i<n){
            if(nums[j]-nums[i]==k){
                s.insert({nums[i],nums[j]});
                i++;
                j++;
            }
            else if(nums[j]-nums[i]>k){
                    i++;
                    if(i==j){
                        j++;
                    }
            }
            else{
                j++;
                if(i==j){
                    j++;
                }
            }
        }
        return s.size();
    }
};
