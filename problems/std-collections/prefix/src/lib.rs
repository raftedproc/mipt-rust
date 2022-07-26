#![forbid(unsafe_code)]

// Rabin-Karp
/*const int SIZE = 1000100;
DataType hashes[SIZE];
DataType pows[SIZE];

DataType hash(const DataType a, const DataType m, const std::string& k)
{
    DataType result = static_cast<DataType>(k[0]) * a;
    auto it = k.begin();
    std::advance(it, 1);
    auto last = k.end();
    std::advance(last, -1);
    for (; it != last; ++it)
    {
        result = ((result + (signed char)*it) * a) % m;
    }

    return (result + k.back()) % m;
}


void initHash(const std::string& target, DataType *h, const DataType a, const DataType mod)
{
    h[0] = 0;
    size_t n = target.length();
    for (size_t i = 1; i <= n; ++i)
    {
        h[i] = (h[i - 1] * a % mod + (signed char)target[i - 1]) % mod;
    }
}

void initPowers(DataType* p, DataType a, DataType mod)
{
    p[0] = 1;
    for (size_t i = 1; i < SIZE; ++i)
        p[i] = p[i - 1] * a % mod;
}

// Make the first pair int
DataType getHash(const DataType l, const DataType r, const DataType *h, const DataType *p, const DataType mod)
{
    return (h[r] + mod - (h[l - 1] * p[r - l + 1]) % mod ) % mod;
}

void rollingWrapper(std::istream& in, std::ostream& out)
{
    DataType a = 0, m = 0;
    size_t n = 0;
    std::string k;
    in >> a >> m >> k >> n;
    if (k.empty())
        return;

    initHash(k, hashes, a, m);
    initPowers(pows, a, m);

    for (size_t i = 0; i < n; ++i)
    {
        DataType l, r;
        in >> l >> r;
        out << getHash(l, r, hashes, pows, m) << "\n";
    }
}
*/

// static SIZE: i32 = 1000100;
use core::cmp::min;

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    // TODO: your code goes here.
    if strs.len() == 0 {
        return String::from("");
    }
    let smallest_word = strs[0];
    let mut common_pref_len = usize::MAX;
    strs.iter().skip(1).for_each(|w| {
        let i = smallest_word.chars();
        let curr_matched_pref_len = w.chars().zip(i).take_while(|p| p.0 == p.1).count();
        // println!("curr_matched_pref_len {}", curr_matched_pref_len);
        common_pref_len = min(curr_matched_pref_len, common_pref_len);
    });
    // println!("smallest prefix len {}", common_pref_len);
    smallest_word
        .chars()
        .take(common_pref_len)
        .collect::<String>()
}

// pub fn longest_common_prefix(strs: Vec<&str>) -> String {
//     // TODO: your code goes here.
//     if strs.len() == 0 {
//         return String::from("");
//     }
//     let smallest_word = strs
//         .iter()
//         .min_by(|x, y| x.chars().count().cmp(&y.chars().count()));
//     // println!("smallest string {}", &smallest_word.unwrap());
//     let mut common_pref_len = usize::MAX;
//     strs.iter().for_each(|w| {
//         let i = smallest_word.unwrap().chars();
//         let curr_matched_pref_len = w.chars().zip(i).take_while(|p| p.0 == p.1).count();
//         // println!("curr_matched_pref_len {}", curr_matched_pref_len);
//         common_pref_len = min(curr_matched_pref_len, common_pref_len);
//     });
//     // println!("smallest prefix len {}", common_pref_len);
//     smallest_word
//         .unwrap()
//         .chars()
//         .take(common_pref_len)
//         .collect::<String>()
// }
