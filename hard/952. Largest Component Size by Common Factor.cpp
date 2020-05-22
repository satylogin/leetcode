class Solution {
public:
    static const int N = 1e5;
    int P[N+5], par[N+5], X[N+5], size[N+5];
    vector<int> v[N+5];
    
    void pre() {
        for (int i = 2; i <= N; ++i) {
            if (P[i]) continue;
            for (int j = i; j <= N; j += i) P[j] = i;
        }
        for (int i = 1; i <= N; ++i) {
            par[i] = i;
            X[i] = 0;
            size[i] = 1;
        }
    }
    
    int find(int x) {
        return (x == par[x] ? x : par[x] = find(par[x]));
    }
    
    void uni(int x, int y) {
        if ((x = find(x)) != (y = find(y))) {
            par[x] = y;
            size[y] += size[x];
        }
    }
    
    int largestComponentSize(vector<int>& A) {
        pre();
        for (auto &it: A) {
            int x = it;
            while (x > 1) {
                int p = P[x];
                if (!X[p]) X[p] = it;
                uni(X[p], it);
                while (!(x % p)) x /= p; 
            }
        }
        int ans = 0;
        for (int i = 1; i <= N; ++i) ans = max(ans, size[find(i)]);
        
        return ans;
    }
};
