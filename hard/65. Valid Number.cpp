class Solution {
public:
    pair<string, string> extract(string &s) {
        string base = "", expo = "";
        int i;
        for (i = 0; s[i] && (s[i] != 'e'); ++i) base += s[i];
        if (s[i] == 'e') i++;
        for (; s[i]; ++i) expo += s[i];
        
        if (base.size() and (base[0] == '+' or base[0] == '-')) {
            base = base.substr(1);
        }
        if (expo.size() and (expo[0] == '+' or expo[0] == '-')) {
            expo = expo.substr(1);
        }
        
        return make_pair(base, expo);
    }
    
    bool valid_base(string &s) {
        int i = 0, point = 0;
        for (; s[i]; ++i) {
            if (s[i] == '.') point++;
            else if (s[i] < '0' || s[i] > '9') return false;
        }
        
        return (point == 0) or (point == 1 and s.size() > 1);
    }
    
    bool valid_expo(string &s) {
        int i = 0;
        for (; s[i]; ++i) {
            if (s[i] < '0' or s[i] > '9') return false;
        }
        
        return true;
    }
    
    bool is_e(string &s) {
        for (auto &it: s) if (it == 'e') return true;
        return false;
    }
    
    string remove_start_space(string &s) {
        string ans = "";
        int i;
        for (i = 0; s[i] && s[i] == ' '; ++i);
        for (; s[i]; ++i) ans += s[i];
        
        return ans;
    }
    
    string trim(string &s) {
        string ans = remove_start_space(s);
        reverse(ans.begin(), ans.end());
        ans = remove_start_space(ans);
        reverse(ans.begin(), ans.end());
        
        return ans;
    }
    
    bool isNumber(string &s) {
        s = trim(s);
        auto it = extract(s);
        bool e = is_e(s);
        bool ans = true;
        if (e) {
            ans &= (it.first != "" and it.second != "");
        }
        return valid_base(it.first) and valid_expo(it.second) and ans and it.first != "";
    }
};
