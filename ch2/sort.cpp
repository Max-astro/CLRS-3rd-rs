#include <vector>
#include <iostream>
#include <random>
#include <chrono>
#include <numeric>
// #include <type_traits>

using namespace std;
using namespace std::chrono;

void mergeSort(vector<int> &nums, int lo, int hi);
void merge(vector<int> &nums, int lo, int mid, int hi);
// template <typename T = int> void pvec(vector<T> &v, string &s);

template <typename T = int, typename D = uniform_int_distribution<int>> class RandomGen {
public:
    RandomGen(int start, int end) : dist(start, end) {
        auto t1 = high_resolution_clock::now().time_since_epoch();
        int s = duration_cast<duration<int>>(t1).count();
        re.seed(s);
    }
    T operator()() { return dist(re); }

private:
    D dist;
    // uniform_real_distribution<double> dist;
    default_random_engine re;
    void seed(int s) { re.seed(s); }
};

template <typename T> void insertionSort(vector<T> &nums) {
    for (int j = 1; j < nums.size(); j++) {
        T j_val = nums[j];
        int i = j - 1;
        while (i >= 0 && nums[i] < j_val) {
            nums[i + 1] = nums[i];
            i--;
        }
        nums[++i] = j_val;
    }
}

// void mergeSort(vector<int> &nums) {
//     return mergeSort(nums, 0, nums.size());
// }
template <typename T = int> void pvec(vector<T> &v, string s) {
    cout << s << " ";
    for (auto i : v) {
        cout << i << " ";
    }
    cout << endl;
}

void mergeSort(vector<int> &nums, int lo, int hi) {
    if (lo < hi - 1) {
        int mid = (lo + hi + 1) / 2;
        mergeSort(nums, lo, mid);
        mergeSort(nums, mid, hi);
        merge(nums, lo, mid, hi);
    }
}

void merge(vector<int> &nums, int lo, int mid, int hi) {
    if (lo >= hi)
        return;
    vector<int> left(nums.begin() + lo, nums.begin() + mid);
    vector<int> right(nums.begin() + mid, nums.begin() + hi);
    left.push_back(numeric_limits<int>::max());
    right.push_back(numeric_limits<int>::max());

    // printf("%d %d %d | %d %d\n", lo, mid, hi, mid - lo, hi - mid);
    // pvec<int>(left, string("left"));
    // pvec<int>(right, string("right"));

    int l = 0, r = 0;
    for (int i = lo; i < hi; i++) {
        if (left[l] < right[r]) {
            nums[i] = left[l++];
        } else {
            nums[i] = right[r++];
        }
    }
}

void InsertMerge(vector<int> &nums, int k, int lo, int mid, int hi) {
    printf("%d %d %d | %d %d\n", lo, mid, hi, mid - lo, hi - mid);
    if (hi - lo <= k) {
        // cout << "ORIGIN: [";
        // for (int j = lo; j < hi; j++) {
        //     cout << nums[j] << " ";
        // }
        // cout << "]\nSTART: [";
        for (int j = lo + 1; j < hi; j++) {
            int num_j = nums[j];
            int i = j - 1;
            while (i >= lo && nums[i] >= num_j) {
                nums[i + 1] = nums[i];
                i--;
            }
            nums[i + 1] = num_j;
        }
        // for (int j = lo; j < hi; j++) {
        //     cout << nums[j] << " ";
        // }
        // cout << "]  INSERT END" << endl;
    } else {
        vector<int> left(nums.begin() + lo, nums.begin() + mid);
        vector<int> right(nums.begin() + mid, nums.begin() + hi);
        pvec<int>(left, string("left"));
        pvec<int>(right, string("right"));
        cout << endl;
        for (int i = lo, l = 0, r = 0; i < hi; i++) {
            if (l == left.size()) {
                nums[i] = right[r++];
            } else if (r == right.size()) {
                nums[i] = left[l++];
            } else {
                nums[i] = right[r] < left[l] ? right[r++] : left[l++];
                // if (right[r] < left[l]) {
                //     nums[i] = right[r++];
                // } else {
                //     nums[i] = left[l++];
                // }
            }
        }
    }
    return;
}

void MergeInsertSort(vector<int> &nums, int k, int lo, int hi) {
    int mid = (lo + hi + 1) / 2;
    // printf("%d %d %d | %d %d\n", lo, mid, hi, mid - lo, hi - mid);
    if (hi - lo >= 5) {
        MergeInsertSort(nums, k, lo, mid);
        MergeInsertSort(nums, k, mid, hi);
    }
    InsertMerge(nums, k, lo, mid, hi);
}

void CountingSort(vector<int> &input, vector<int> &output, int _max) {
    vector<int> count(_max, 0);

    for (int i : input) {
        count[i]++;
    }
    for (int i = 1, last = count[0]; i != _max; i++) {
        count[i] += last;
        last = count[i];
    }

    for (int i = input.size() - 1; i >= 0; i--) {
        output[count[input[i]]] = input[i];
        count[input[i]]--;
    }
}

void RadixSort(vector<int> &input, int _maxIndex) {
    for (int i = 0; i != _maxIndex; i++) {
    }
}

int main() {
    // RandomGen<double, uniform_real_distribution<double>> rng(-1000, 1000);
    RandomGen rng(0, 1000);

    vector<int> a(25, 0);
    vector<int> b(25, 0);
    for (auto i = a.begin(); i != a.end(); i++) {
        *i = rng();
    }

    for (auto i : a)
        cout << i << " ";
    cout << endl;

    // vector<int> b(a.begin() + 1, a.begin() + 3);
    // for (auto i : b)
    //     cout << i << " ";
    // cout << endl;

    // vector<int> b(a.begin() + 1, a.begin() + a.size() / 2);
    // insertionSort(a);
    // mergeSort(a);

    CountingSort(a, b, 1000);
    for (auto i : b)
        cout << i << " ";
    cout << endl;
}