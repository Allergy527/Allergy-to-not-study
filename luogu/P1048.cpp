/*@author    Allergy
 *@email     Allergy527@gmail.com
 *@workspace luogu\P1048.cpp
 *@data      2023/11/26 13:41:51
 */
#include<bits/stdc++.h>
#define ll long long

using namespace std;

void solve() {
    int n, v, i, j;
    cin >> v >> n;
    vector<vector<int>>qwq(n + 1, vector<int>(v + 1));
    vector<int>qwqV(n+1,0);
    vector<int>qwqW(n+1,0);
    //输入物品及价值
    for(i = 1;i <= n;++i)cin >> qwqW[i] >> qwqV[i];
    //动态规划
    for(i = 1;i <= n;++i) {
        for(j = 1;j <= v;++j) {
            if(qwqW[i] > j)qwq[i][j] = qwq[i - 1][j];
            else qwq[i][j] = max(qwq[i - 1][j], qwq[i - 1][j - qwqW[i]] + qwqV[i]);
        }
    }
    cout << qwq[n][v] << '\n';
}

int main() {
    ios::sync_with_stdio(false);
    int t = 1;
    //cin >> t;
    while(t--)solve();
    cin>>t;
    return 0;
}