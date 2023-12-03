/*@author    Allergy
 *@email     Allergy527@gmail.com
 *@workspace Atcoder\12.2\E.cpp
 *@data      2023/12/02 20:54:56
 */
#include<bits/stdc++.h>
#define ll long long

using namespace std;

void solve() {
    int n, m, i, j,l,c,d,max=0;
    cin >> n >> m >> l;
    vector<int>qwq(n);
    vector<int>qaq(m);
    vector<vector<int>>qvq(n, vector<int>(m));
    for(auto &x : qwq)cin >> x;
    for(auto &x : qaq)cin >> x;
    for(i = 0;i < n;++i)
        for(j = 0;j < m;++j)
            qvq[i][j] = qwq[i] + qaq[j];
    for(i = 0;i < l;++i) {
        cin >> c >> d;
        qvq[c-1][d-1] = 0;
    }
    for(i = 0;i < n;++i)
        for(j = 0;j < m;++j)
            if(qvq[i][j] > max)max = qvq[i][j];
    cout << max;
}

int main() {
    ios::sync_with_stdio(false);
    int t = 1;
    //cin >> t;
    while(t--)solve();
    cin>>t;
    return 0;
}