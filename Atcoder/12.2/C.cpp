/*@author    Allergy
 *@email     Allergy527@gmail.com
 *@workspace Atcoder\12.2\C.cpp
 *@data      2023/12/02 20:21:51
 */
#include<bits/stdc++.h>
#define ll long long

using namespace std;

ll Bisect_right(vector<ll> list, ll num) {//右偏的二分
    ll low = 0, high = list.size() - 1;
    while(low < high) {
        ll mid = (low + high+1)>> 1;
        if(list[mid] >= num)high = mid - 1;
        else low = mid;
    }
    return low;
}
void solve() {
    ll n, m, i, j,Len=0;
    cin >> n;
    vector<ll>qwq(n);
    for(auto &x : qwq)cin >> x;
    vector<ll>qvq(qwq);
    map<int, int>ck;
    sort(qvq.begin(), qvq.end());
    ck[qvq[0]+1] = 0;
    for(i = 1;i < n;++i) {
        if(qvq[i] + 1 != qvq[i - 1] + 1) {
            ck[qvq[i] + 1] = ck[qvq[i - 1] + 1];
        }
        ++ck[qvq[i]+1];
    }
    vector<ll>qaq(n);
    qaq[0] = qvq[0];
    for(i = 1;i < n;++i)qaq[i]=qvq[i]+qaq[i - 1];
    for(auto x : qwq) {
        cout << qaq[n-1]-qaq[ck[x+1]]<<' ';
    }

}

int main() {
    ios::sync_with_stdio(false);
    int t = 1;
    //cin >> t;
    while(t--)solve();
    cin>>t;
    return 0;
}