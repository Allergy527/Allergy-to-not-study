#include<bits/stdc++.h>

using namespace std;

void MERGE_SORT(vector<int> &, int, int);

int main() {
    int n;
    cout << "Please enter the lenth of the list:";
    cin >> n;
    vector<int> qwq(n);
    for(auto &x : qwq)cin >> x;
    MERGE_SORT(qwq, 0, n - 1);
    for(auto &x : qwq)cout << x << " ";
    getchar();getchar();
    return 0;
}

void MERGE_SORT(vector<int> &list, int p, int r) {
    int q = (p + r) / 2;//二分
    if(p >= r)return;//当左大于右的时候退出
    MERGE_SORT(list, p, q);//分裂左边
    MERGE_SORT(list, q + 1, r);//分裂右边
    //下面是合并函数，干脆写在一起了。
    vector<int>L, R;//记录左右子树？可以这么理解罢。
    int a = 0, b = 0;//两个指针，记录左右各自取到哪里了。
    for(int x = p;x <= q;++x)L.emplace_back(list[x]);
    for(int x = q + 1;x <= r;++x)R.emplace_back(list[x]);
    for(int k = p;k <= r;++k) {
        if(a == q + 1 - p) list[k] = R[b++];//如果L取完了，那就只取R了
        else if(b == r - q) list[k] = L[a++];//如果R取完了，那就只取L了
        else if(L[a] < R[b]) list[k] = L[a++];//取小的那一个放
        else list[k] = R[b++];
    }
}