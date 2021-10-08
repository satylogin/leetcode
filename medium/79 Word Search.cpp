class Solution {
public:
    bool isSafe(vector<vector<char>>& board, string word,int x,int y,int n,int m,int idx,int N,vector<vector<bool>>&visit){
        if(x>=0&&x<n&&y>=0&&y<m&&idx<N&&board[x][y]==word[idx]&&visit[x][y]==false){
            return true;
        }
        return false;
    }
    bool dfs(vector<vector<char>>& board, string word,int x,int y,int n,int m,int idx,int N,vector<vector<bool>>&visit){
        if(idx==N-1&&board[x][y]==word[idx]){
            //cout<<"end"<<endl;
            return true;
        }
        //cout<<idx<<" "<<word[idx]<<endl;
        visit[x][y]=true;
        int cx[]={1,-1,0,0};
        int cy[]={0,0,1,-1};
        if(word[idx]!=board[x][y]){
            return false;
        }
        int i;
        bool f=false;
        for(i=0;i<4;i++){
            int newx=x+cx[i];
            int newy=y+cy[i];
            if(isSafe(board,word,newx,newy,n,m,idx+1,N,visit)){
                f=dfs(board,word,newx,newy,n,m,idx+1,N,visit);
                //cout<<f<<endl;
            }
            if(f){
                return f;
            }
        }
        visit[x][y]=false;
        return f;
    }
    
    bool dfs_util(vector<vector<char>>& board, string word,int x,int y,int n,int m,int idx,int N,vector<vector<bool>>visit){
        bool f=dfs(board,word,x,y,n,m,idx,N,visit);
        return f;
    }
    bool exist(vector<vector<char>>& board, string word) {
       int i,j,n=board.size(),m=board[0].size(),N=word.size();
        vector<vector<bool>>visit(n);
        for(i=0;i<n;i++){
            for(j=0;j<m;j++){
                visit[i].push_back(false);
            }
        }
        bool f=false;
        for(i=0;i<n;i++){
            for(j=0;j<m;j++){
                if(board[i][j]==word[0]){
                    //cout<<"call dfs"<<endl;
                    f=dfs_util(board,word,i,j,n,m,0,N,visit);      
                }
                if(f){
                    return f;
                }
            }
        }
        return f;
    }
};
