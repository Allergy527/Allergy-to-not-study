/*@author    Allergy
 *@email     Allergy527@gmail.com
 *@workspace Cpp
 *@data      2023/11/20 12:46:04
 */
#include<bits/stdc++.h>
#define ll long long

using namespace std;

void solve() {
    int n, m, i, j;
    cin >> n;
    vector<int>qwq;
    //下面是分组循环
    i = 0;
    int mx = 0;
    while(i < n) { //比如下面这段就是用来求解连续子段的最长长度
        int st = i;
        ++i;
        //遇到什么情况需要分开求
        while(i < n && qwq[i] - qwq[i + 1] == 1)++i;
        mx = max(mx, st - i + 1);
    }
}

int main() {
    ios::sync_with_stdio(false);
    int t = 1;
    //cin>>t;
    while(t--)solve();
    cin>>t;
    return 0;
}